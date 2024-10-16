use std::sync::{Arc, Mutex};

use common::no_hash_hashmap::NoHashHashMap;
use type_heap::TYPE_HEAP;

use crate::section_manager::LoadedType;

pub mod type_heap;

#[derive(Clone, Copy, PartialEq)]
enum Primitive {
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

pub enum Object {
    Primitive(Primitive),
    HeapObject(HeapObject),
}
struct HeapObject {
    data: Arc<[u8]>,
    type_id: usize,
}

impl HeapObject {
    pub fn set_field(&self, field: Option<u64>, other: HeapObject) {
        if let Some(field) = field {
            let target = unsafe {
                std::slice::from_raw_parts_mut(self.data.as_ptr() as *mut u8, self.data.len())
            };
            let type_heap = TYPE_HEAP.lock().unwrap();
            let structure = match type_heap.index(self.type_id) {
                type_heap::Type::Structure(structure) => structure,
                type_heap::Type::Interface(..) => panic!("interface can't have fields"),
            };
            type_heap.traverse_and_set(structure.field(field), &other.data, target);
        } else {
            let target = unsafe {
                std::slice::from_raw_parts_mut(self.data.as_ptr() as *mut u8, self.data.len())
            };
            let type_heap = TYPE_HEAP.lock().unwrap();
            let structure = match type_heap.index(self.type_id) {
                type_heap::Type::Structure(structure) => structure,
                type_heap::Type::Interface(..) => panic!("interface can't have fields"),
            };
            structure
                .field_iter()
                .for_each(|f| type_heap.traverse_and_set(f, &other.data, target));
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
                let type_id = type_heap.from_hash(hash);
                Self::HeapObject(HeapObject {
                    type_id,
                    data: Arc::from(vec![0; type_heap.index(type_id).size()]),
                })
            }
        }
    }

    /// Set a field of this object
    pub fn set(&mut self, field: Option<u64>, other: Object) {
        match field {
            Some(field) => match (self, other) {
                (Self::Primitive(..), _) => panic!("primitive object can't have fields"),
                (Self::HeapObject(object), Self::HeapObject(other)) => {
                    object.set_field(Some(field), other);
                }
                (Self::HeapObject(..), Self::Primitive(..)) => {
                    panic!("the source object does not match the target")
                }
            },
            None => match (self, other) {
                (Self::Primitive(ref mut primitive), Self::Primitive(v)) => {
                    *primitive = v;
                }
                (Self::HeapObject(object), Self::HeapObject(other)) => {
                    object.set_field(None, other);
                }
                _ => panic!("the source object does not match the target"),
            },
        }
    }
}
