use crate::section_manager::LoadedType;

use super::objects::Object;

#[derive(Clone, Copy, Debug)]
struct LocalValue {
    start: usize,
    size: usize,
}

#[derive(Debug)]
pub struct LocalVariables {
    objects: Vec<Object>,
    current: LocalValue,
    local_stack: Vec<LocalValue>,
}

impl LocalVariables {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            current: LocalValue { start: 0, size: 0 },
            local_stack: Vec::new(),
        }
    }

    pub fn new_local(&mut self, size: usize) {
        let old = self.current;
        self.local_stack.push(self.current);
        self.current = LocalValue {
            start: old.start + old.size,
            size,
        };
        if self.current.start + size > self.objects.len() {
            let exceed = self.current.start + size - self.objects.len();
            for _ in 0..exceed {
                self.objects.push(Object::new(LoadedType::Void));
            }
        }
    }

    pub fn set(&mut self, index: usize, object: Object) {
        match self.objects.get_mut(self.current.start + index) {
            Some(value) => {
                *value = object;
            }
            None => {
                panic!("Out of bound local");
            }
        }
    }

    pub fn get(&mut self, index: usize) -> Object {
        match self.objects.get(self.current.start + index) {
            Some(value) => value.clone(),
            None => {
                panic!("Out of bound local");
            }
        }
    }

    pub fn restore_local(&mut self) {
        self.objects[self.current.start..(self.current.start + self.current.size)]
            .fill(Object::new(LoadedType::Void));
        let saved = self.local_stack.pop().expect("Refactor this");
        self.current = saved;
    }
}
