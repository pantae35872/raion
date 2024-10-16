use std::{collections::hash_map::Values, ops::Index, sync::Mutex};

use common::no_hash_hashmap::NoHashHashMap;
use lazy_static::lazy_static;

use crate::section_manager::{LoadedProcedure, LoadedStructure};

lazy_static! {
    pub static ref TYPE_HEAP: Mutex<TypeHeap> = Mutex::new(TypeHeap::new());
}

pub struct TypeHeap {
    types: Vec<Type>,
    type_map: NoHashHashMap<u64, usize>,
}

pub enum Type {
    Interface(Interface),
    Structure(Structure),
}

struct Interface {}

pub struct Structure {
    fields: NoHashHashMap<u64, Field>,
    size: usize,
    procedures: NoHashHashMap<u64, LoadedProcedure>,
}

#[derive(Debug, Clone)]
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

impl TypeHeap {
    fn new() -> Self {
        Self {
            types: Vec::new(),
            type_map: NoHashHashMap::default(),
        }
    }

    fn set(&self, target: &mut [u8], source: &[u8], field: &Field, s_offset: usize) {
        match field {
            Field::Custom { index, offset } => match &self.types[*index] {
                Type::Structure(structure) => {
                    for field in structure.fields.values() {
                        self.set(target, source, field, *offset);
                    }
                }
                _ => unimplemented!(),
            },
            Field::Primitive { typ, offset } => {
                let range = (*offset + s_offset)..(*offset + typ.size());
                target[range.clone()].copy_from_slice(&source[range]);
            }
        }
    }

    pub fn traverse_and_set(&self, field: &Field, source: &[u8], target: &mut [u8]) {
        self.set(target, source, field, 0);
    }

    pub fn index(&self, index: usize) -> &Type {
        &self.types[index]
    }

    pub fn from_hash(&self, hash: u64) -> usize {
        *self.type_map.get(&hash).expect("Type does not exists")
    }
}
