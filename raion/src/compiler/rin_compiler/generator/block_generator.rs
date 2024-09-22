use std::collections::HashMap;

use crate::compiler::rin_compiler::{Block, Type};

use super::{GeneratorError, Variable, Variables};

pub struct BlockGenerator<'a> {
    stack_loc: &'a mut usize,
    variables: &'a Variables,
    local_variables: Variables,
    body: String,
}

impl<'a> BlockGenerator<'a> {
    pub fn new(variables: &'a Variables, stack_loc: &'a mut usize) -> Self {
        Self {
            variables,
            stack_loc,
            local_variables: Variables::new(),
            body: String::new(),
        }
    }

    pub fn gen_block<'b>(
        &mut self,
        block: &'b Block,
    ) -> Result<(Type, String), GeneratorError<'b>> {
        let mut return_type = Type::Void;
        todo!();
        //for statement in block.body.iter() {
        //    if let Some(stmt_type) = self.gen_statement(statement)? {
        //        if return_type != Type::Void && return_type != stmt_type {
        //            return Err(GeneratorError::UnexpectedType {
        //                expected: return_type,
        //                unexpected: stmt_type,
        //            });
        //        }
        //        return_type = stmt_type;
        //    }
        //}
        return Ok((return_type, self.body));
    }
}
