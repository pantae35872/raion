pub mod constants;
pub mod memory;
pub mod no_hash_hashmap;
pub mod register;
pub mod sin;

pub trait VecUtils<T: Clone> {
    fn insert_slice(&mut self, index: usize, slice: &[T]);
}

impl<T: Clone> VecUtils<T> for Vec<T> {
    fn insert_slice(&mut self, index: usize, slice: &[T]) {
        assert!(index <= self.len(), "Index out of bounds");
        self.reserve(slice.len());
        self.splice(index..index, slice.iter().cloned());
    }
}

#[macro_export]
macro_rules! inline_if {
    ($condition:expr, $true_expr:expr, $false_expr:expr) => {
        if $condition {
            $true_expr
        } else {
            $false_expr
        }
    };
}
