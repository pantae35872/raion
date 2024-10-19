use std::{any::Any, mem, sync::Arc};

use common::memory::buffer_reader::BufferReader;
use heap_object::HeapObjectData;
use type_heap::{Field, PrimitiveType, TYPE_HEAP};

use crate::section_manager::LoadedType;

pub mod heap_object;
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
    inner: HeapObjectData,
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

    fn from_type_and_buf(typ: &PrimitiveType, buf: &[u8]) -> Self {
        match typ {
            PrimitiveType::U8 => Self::U8(BufferReader::new(buf).read_u8().unwrap()),
            PrimitiveType::U16 => Self::U16(BufferReader::new(buf).read_u16().unwrap()),
            PrimitiveType::U32 => Self::U32(BufferReader::new(buf).read_u32().unwrap()),
            PrimitiveType::U64 => Self::U64(BufferReader::new(buf).read_u64().unwrap()),
            PrimitiveType::I64 => Self::I64(BufferReader::new(buf).read_i64().unwrap()),
            PrimitiveType::I32 => Self::I32(BufferReader::new(buf).read_i32().unwrap()),
            PrimitiveType::I16 => Self::I16(BufferReader::new(buf).read_i16().unwrap()),
            PrimitiveType::I8 => Self::I8(BufferReader::new(buf).read_i8().unwrap()),
            PrimitiveType::Bool => Self::Bool(BufferReader::new(buf).read_u8().unwrap() == 1),
            PrimitiveType::Void => Self::Void,
        }
    }
}

impl HeapObject {
    pub fn set_field(&mut self, field: Option<u64>, other: Object) {
        if let Some(field) = field {
            let type_heap = TYPE_HEAP.read().unwrap();
            let structure = match type_heap.index(self.inner.type_id()) {
                type_heap::Type::Structure(structure) => structure,
                type_heap::Type::Interface(..) => panic!("interface can't have fields"),
            };
            let target = unsafe { self.inner.data_mut() };
            type_heap.set(target, other, structure.field(field));
        } else {
            let other = match other {
                Object::HeapObject(other) => other,
                Object::Primitive(..) => panic!("Cannot set primetives to heap object"),
            };
            self.inner = other.inner.clone();
        }
    }

    pub fn get_field(&self, field: u64) -> Object {
        let type_heap = TYPE_HEAP.read().unwrap();
        let structure = match type_heap.index(self.inner.type_id()) {
            type_heap::Type::Structure(structure) => structure,
            type_heap::Type::Interface(..) => panic!("interface can't have fields"),
        };
        let data = self.inner.data();
        match structure.field(field) {
            Field::Custom { offset, .. } => {
                let data = &data[*offset..(*offset + size_of::<HeapObjectData>())];
                if !data.iter().all(|&x| x == 0) {
                    let data: HeapObjectData = unsafe {
                        core::mem::transmute_copy(
                            &<[u8; size_of::<HeapObjectData>()]>::try_from(data).unwrap(),
                        )
                    };
                    unsafe {
                        data.increase_ref_count();
                    }
                    Object::HeapObject(HeapObject { inner: data })
                } else {
                    panic!("Null pointer exception (java reference)");
                }
            }
            Field::Primitive { typ, offset } => Object::Primitive(Primitive::from_type_and_buf(
                typ,
                &data[*offset..(*offset + typ.size())],
            )),
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
                let type_heap = TYPE_HEAP.read().unwrap();
                let type_id = type_heap.from_hash(hash).unwrap();
                drop(type_heap); // Prevents dead lock
                Self::HeapObject(HeapObject {
                    inner: HeapObjectData::new(type_id),
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

    pub fn get(&mut self, field: u64) -> Object {
        match self {
            Self::Primitive(_) => panic!("primetives object can't have fields"),
            Self::HeapObject(object) => object.get_field(field),
        }
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
