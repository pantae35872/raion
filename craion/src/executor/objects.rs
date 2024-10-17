use std::{mem, sync::Arc};

use type_heap::TYPE_HEAP;

use crate::section_manager::LoadedType;

pub mod type_heap;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Primitive {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Bool(bool),
    Void,
}

#[derive(Debug, Clone)]
pub enum Object {
    Primitive(Primitive),
    HeapObject(HeapObject),
}

#[derive(Clone, Debug)]
struct HeapObject {
    data: Arc<[u8]>,
    type_id: usize,
}

impl Primitive {
    fn to_bytes(&self, buffer: &mut Vec<u8>) {
        match self {
            Self::Void => {}
            Self::Bool(booo) => buffer.push(*booo as u8),
            Self::U8(value) => buffer.push(*value),
            Self::U16(value) => buffer.extend_from_slice(&value.to_le_bytes()),
            Self::U32(value) => buffer.extend_from_slice(&value.to_le_bytes()),
            Self::U64(value) => buffer.extend_from_slice(&value.to_le_bytes()),
            Self::I8(value) => buffer.extend_from_slice(&value.to_le_bytes()),
            Self::I16(value) => buffer.extend_from_slice(&value.to_le_bytes()),
            Self::I32(value) => buffer.extend_from_slice(&value.to_le_bytes()),
            Self::I64(value) => buffer.extend_from_slice(&value.to_le_bytes()),
        }
    }
}

impl HeapObject {
    pub fn set_field(&self, field: Option<u64>, other: Object) {
        if let Some(field) = field {
            let target = unsafe {
                std::slice::from_raw_parts_mut(self.data.as_ptr() as *mut u8, self.data.len())
            };
            let type_heap = TYPE_HEAP.lock().unwrap();
            let structure = match type_heap.index(self.type_id) {
                type_heap::Type::Structure(structure) => structure,
                type_heap::Type::Interface(..) => panic!("interface can't have fields"),
            };
            match other {
                Object::HeapObject(object) => {
                    type_heap.traverse_and_set(structure.field(field), &object.data, target, 0);
                }
                Object::Primitive(primetive) => {
                    let mut buf = Vec::new();
                    primetive.to_bytes(&mut buf);
                    let (old_offset, new_field) = structure.field(field).replace_offset(0);
                    type_heap.traverse_and_set(&new_field, &mut buf, target, old_offset);
                }
            }
        } else {
            let target = unsafe {
                std::slice::from_raw_parts_mut(self.data.as_ptr() as *mut u8, self.data.len())
            };
            let type_heap = TYPE_HEAP.lock().unwrap();
            let structure = match type_heap.index(self.type_id) {
                type_heap::Type::Structure(structure) => structure,
                type_heap::Type::Interface(..) => panic!("interface can't have fields"),
            };
            structure.field_iter().for_each(|f| match &other {
                Object::HeapObject(object) => {
                    type_heap.traverse_and_set(f, &object.data, target, 0);
                }
                Object::Primitive(..) => unreachable!("This method can only be called on heaped object and setting a primetive to a heap object with no field is consider and invalid"),
            });
        }
    }
}

impl Object {
    pub fn new(typ: LoadedType) -> Self {
        match typ {
            LoadedType::U8 => Self::Primitive(Primitive::U8(0)),
            LoadedType::U16 => Self::Primitive(Primitive::U16(0)),
            LoadedType::U32 => Self::Primitive(Primitive::U32(0)),
            LoadedType::U64 => Self::Primitive(Primitive::U64(0)),
            LoadedType::I8 => Self::Primitive(Primitive::I8(0)),
            LoadedType::I16 => Self::Primitive(Primitive::I16(0)),
            LoadedType::I32 => Self::Primitive(Primitive::I32(0)),
            LoadedType::I64 => Self::Primitive(Primitive::I64(0)),
            LoadedType::Void => Self::Primitive(Primitive::Void),
            LoadedType::Bool => Self::Primitive(Primitive::Bool(false)),
            LoadedType::Custom { hash } => {
                let type_heap = TYPE_HEAP.lock().unwrap();
                let type_id = type_heap.from_hash(hash).unwrap();
                Self::HeapObject(HeapObject {
                    type_id,
                    data: Arc::from(vec![0; type_heap.index(type_id).size()]),
                })
            }
        }
    }

    pub fn set_primtive(&mut self, prim: Primitive) {
        match self {
            Self::HeapObject(..) => {
                panic!("cannot set primitive on an object that is not primtive")
            }
            Self::Primitive(ref mut primtive) => {
                if mem::discriminant(primtive) != mem::discriminant(&prim) {
                    panic!("the provided type does not match in an object");
                }
                *primtive = prim;
            }
        };
    }

    /// Set a field of this object
    pub fn set(&mut self, field: Option<u64>, other: Object) {
        match field {
            Some(field) => match self {
                Self::Primitive(..) => panic!("primitive object can't have fields"),
                Self::HeapObject(object) => {
                    object.set_field(Some(field), other);
                }
            },
            None => match (self, other) {
                (Self::Primitive(ref mut primitive), Self::Primitive(v)) => {
                    *primitive = v;
                }
                (Self::HeapObject(object), Self::HeapObject(other)) => {
                    object.set_field(None, Object::HeapObject(other));
                }
                _ => panic!("the source object does not match the target"),
            },
        }
    }
}
