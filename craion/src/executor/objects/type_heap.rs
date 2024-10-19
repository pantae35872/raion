use std::{
    collections::hash_map::Values,
    sync::{Arc, Mutex, RwLock},
};

use common::no_hash_hashmap::NoHashHashMap;
use lazy_static::lazy_static;

use crate::section_manager::{LoadedProcedure, LoadedStructure, LoadedType, SectionManager};

use super::{heap_object::HeapObjectData, Object};

lazy_static! {
    pub static ref TYPE_HEAP: RwLock<TypeHeap> = RwLock::new(TypeHeap::new());
}

pub struct TypeHeap {
    types: Vec<Type>,
    type_map: NoHashHashMap<u64, usize>,
}

#[derive(Debug)]
pub enum Type {
    Interface(Interface),
    Structure(Structure),
}

#[derive(Debug)]
pub struct Interface {}

#[derive(Debug)]
pub struct Structure {
    fields: NoHashHashMap<u64, Field>,
    size: usize,
    procedures: NoHashHashMap<u64, LoadedProcedure>,
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Void,
    Bool,
}

#[derive(Debug)]
pub enum Field {
    Primitive { typ: PrimitiveType, offset: usize },
    Custom { index: usize, offset: usize },
}

impl PrimitiveType {
    pub fn size(&self) -> usize {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 => 4,
            Self::U64 | Self::I64 => 8,
            Self::Bool => 1,
            Self::Void => 0,
        }
    }

    pub fn from_loaded(loaded: &LoadedType) -> Self {
        match loaded {
            LoadedType::U64 => Self::U64,
            LoadedType::I64 => Self::I64,
            LoadedType::U32 => Self::U32,
            LoadedType::I32 => Self::I32,
            LoadedType::U16 => Self::U16,
            LoadedType::I16 => Self::I16,
            LoadedType::U8 => Self::U8,
            LoadedType::I8 => Self::I8,
            LoadedType::Bool => Self::Bool,
            LoadedType::Void => Self::Void,
            LoadedType::Custom { .. } => panic!("cannot create primetive type from defined type"),
        }
    }
}

impl Structure {
    pub fn field(&self, hash: u64) -> &Field {
        self.fields.get(&hash).expect("Field does not exists")
    }

    pub fn field_iter(&self) -> Values<'_, u64, Field> {
        self.fields.values()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Self::Structure(structure) => structure.size(),
            Self::Interface(..) => todo!(),
        }
    }
}

impl Field {
    pub fn replace_offset(&self, new_offset: usize) -> (usize, Field) {
        match self {
            Self::Primitive { typ, offset } => (
                *offset,
                Field::Primitive {
                    typ: typ.clone(),
                    offset: new_offset,
                },
            ),
            Self::Custom { offset, index } => (
                *offset,
                Field::Custom {
                    index: *index,
                    offset: new_offset,
                },
            ),
        }
    }
}

impl TypeHeap {
    fn new() -> Self {
        Self {
            types: Vec::new(),
            type_map: NoHashHashMap::default(),
        }
    }

    pub fn init(&mut self, manager: &SectionManager) {
        for (hash, structure) in manager.structure_iter() {
            self.parse_struct(*hash, structure, manager);
        }
    }

    fn parse_struct(
        &mut self,
        name: u64,
        structure: &LoadedStructure,
        manager: &SectionManager,
    ) -> usize {
        if let Some(index) = self.from_hash(name) {
            return index;
        }
        let mut fields = NoHashHashMap::default();
        let mut offset = 0;
        for (field_name, field) in structure.fields.iter() {
            match &field.contain_type {
                LoadedType::Custom { hash } => {
                    let index =
                        self.parse_struct(*hash, manager.structure(*hash).unwrap(), manager);
                    let stru = &self.types[index];
                    let p_offset = offset;
                    offset += size_of::<HeapObjectData>(); // pointer size for reference to that structure
                    fields.insert(
                        *field_name,
                        Field::Custom {
                            index,
                            offset: p_offset,
                        },
                    );
                }
                primetive => {
                    let typ = PrimitiveType::from_loaded(primetive);
                    let p_offset = offset;
                    offset += typ.size();
                    fields.insert(
                        *field_name,
                        Field::Primitive {
                            typ,
                            offset: p_offset,
                        },
                    );
                }
            }
        }

        self.types.push(Type::Structure(Structure {
            fields,
            size: offset,
            procedures: structure.procs.clone(),
        }));

        self.type_map.insert(name, self.types.len() - 1);
        return self.types.len() - 1;
    }

    pub fn set(&self, target: &mut [u8], source: Object, field: &Field) {
        match field {
            Field::Custom { index, offset } => match (&self.types[*index], source) {
                (Type::Structure(..), Object::HeapObject(source)) => {
                    let old = &target[*offset..(*offset + size_of::<HeapObjectData>())];
                    if !old.iter().all(|&x| x == 0) {
                        let old: HeapObjectData = unsafe {
                            core::mem::transmute_copy(
                                &<[u8; size_of::<HeapObjectData>()]>::try_from(old).unwrap(),
                            )
                        };
                        drop(old);
                    }
                    let data_bytes: [u8; size_of::<HeapObjectData>()] =
                        unsafe { std::mem::transmute(source.inner) };
                    target[*offset..(*offset + size_of::<HeapObjectData>())]
                        .copy_from_slice(&data_bytes);
                }
                _ => unimplemented!(),
            },
            Field::Primitive { typ, offset } => {
                let range = *offset..(*offset + typ.size());
                let source = match source {
                    Object::Primitive(primetive) => primetive,
                    Object::HeapObject(..) => panic!("cannot set heap object to primetive"),
                };

                let mut buf = Vec::new();
                source.to_bytes(&mut buf);
                target[range.clone()].copy_from_slice(&buf);
            }
        }
    }

    pub fn index(&self, index: usize) -> &Type {
        &self.types[index]
    }

    pub fn from_hash(&self, hash: u64) -> Option<usize> {
        self.type_map.get(&hash).as_deref().copied()
    }
}
