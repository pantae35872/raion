use std::{
    alloc::{Allocator, Global, Layout},
    fmt::Debug,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::type_heap::{Field, Type, TYPE_HEAP};

#[repr(C)]
struct HeapObjectDataInner {
    reference_count: AtomicUsize,
    data: [u8],
}

pub struct HeapObjectData {
    inner: NonNull<HeapObjectDataInner>,
    type_id: usize,
}

impl HeapObjectData {
    fn layout(size: usize) -> Layout {
        let layout_atomic = Layout::new::<AtomicUsize>();
        let layout_data = Layout::array::<u8>(size).unwrap();
        let layout_dst = layout_atomic.extend(layout_data).unwrap().0;

        layout_dst.pad_to_align()
    }

    pub fn new(type_id: usize) -> Self {
        let data_size = TYPE_HEAP.read().unwrap().index(type_id).size();
        let inner = Global
            .allocate_zeroed(Self::layout(data_size))
            .unwrap()
            .as_ptr() as *mut HeapObjectDataInner;
        // Create a dummy pointer that have a metadata of size data_size
        let dummy = unsafe { std::slice::from_raw_parts(0xdeadbeaf as *const u8, data_size) };
        // replace the fat pointer metadata with the dummy metadata
        let inner = inner.with_metadata_of(dummy) as *mut HeapObjectDataInner;
        unsafe {
            (*inner).reference_count = AtomicUsize::new(1);
        }
        Self {
            inner: NonNull::new(inner).unwrap(),
            type_id,
        }
    }

    pub unsafe fn increase_ref_count(&self) {
        (*self.inner.as_ptr())
            .reference_count
            .fetch_add(1, Ordering::SeqCst);
    }

    pub fn data(&self) -> &[u8] {
        unsafe { &(*self.inner.as_ptr()).data }
    }

    pub unsafe fn data_mut(&mut self) -> &mut [u8] {
        unsafe { &mut (*self.inner.as_ptr()).data }
    }

    pub fn type_id(&self) -> usize {
        self.type_id
    }
}

impl Debug for HeapObjectData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HeapObjectData {{ data: {:?}, type_id: {} }}",
            unsafe { &(*self.inner.as_ptr()).data },
            self.type_id
        )
    }
}

impl Clone for HeapObjectData {
    fn clone(&self) -> Self {
        unsafe {
            (*self.inner.as_ptr())
                .reference_count
                .fetch_add(1, Ordering::SeqCst);
        };
        Self {
            inner: self.inner.clone(),
            type_id: self.type_id,
        }
    }
}

impl Drop for HeapObjectData {
    fn drop(&mut self) {
        let inner = unsafe { &*self.inner.as_ptr() };
        if inner.reference_count.fetch_sub(1, Ordering::SeqCst) == 1 {
            let type_heap = TYPE_HEAP.read().unwrap();
            let structure = match type_heap.index(self.type_id) {
                Type::Structure(structure) => structure,
                _ => todo!(),
            };
            for field in structure.field_iter() {
                match field {
                    Field::Custom { offset, .. } => {
                        let other = &inner.data[*offset..(*offset + size_of::<HeapObjectData>())];
                        if !other.iter().all(|&x| x == 0) {
                            let other: HeapObjectData = unsafe {
                                core::mem::transmute_copy(
                                    &<[u8; size_of::<HeapObjectData>()]>::try_from(other).unwrap(),
                                )
                            };
                            drop(other);
                        }
                    }
                    _ => continue,
                }
            }
            println!("Deallocating: {}", self.type_id);
            unsafe {
                Global.deallocate(
                    self.inner.cast(),
                    Layout::from_size_align(size_of::<AtomicUsize>() + inner.data.len(), 8)
                        .expect("Should not failed"),
                );
            }
        }
    }
}
