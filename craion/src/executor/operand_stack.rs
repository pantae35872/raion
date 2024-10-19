use super::objects::Object;

#[derive(Debug)]
pub struct OperandStack {
    op_stack: Vec<Object>,
}

impl OperandStack {
    pub fn new() -> Self {
        Self {
            op_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Object) {
        self.op_stack.push(object);
    }

    pub fn pop(&mut self) -> Object {
        self.op_stack.pop().expect("Stack underflow")
    }

    /// Add top two object if the object is primetive and addable or two of them implements Add interface
    pub fn add(&mut self) {
        let rhs = self.op_stack.pop().expect("Stack underflow error");
        let lhs = self.op_stack.pop().expect("Stack underflow error");
        match (lhs, rhs) {
            (Object::Primitive(lhs), Object::Primitive(rhs)) => todo!(),
            _ => todo!(),
        }
    }
}
