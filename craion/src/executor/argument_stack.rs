use super::{objects::Object, ProgramState};

#[derive(Debug)]
pub struct ArgumentStack {
    argument: Vec<Object>,
}

impl ArgumentStack {
    pub fn new() -> Self {
        Self {
            argument: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Object) {
        self.argument.push(object);
    }

    pub fn load(&mut self, index: usize, state: &mut ProgramState) {
        let object = self.argument.pop().expect("Not enough argument");
        state.local.set(index, object);
    }
}
