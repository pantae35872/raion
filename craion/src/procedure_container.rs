use common::no_hash_hashmap::NoHashHashMap;

use crate::{
    executor::{ExecutorState, ProgramState},
    section_manager::LoadedProcedure,
};

#[derive(Debug)]
pub struct ProcedureContainer {
    procedures: NoHashHashMap<u64, LoadedProcedure>,
}

impl ProcedureContainer {
    pub fn new() -> Self {
        Self {
            procedures: NoHashHashMap::default(),
        }
    }

    pub fn load(&mut self, key: u64, value: LoadedProcedure) {
        self.procedures.insert(key, value);
    }

    pub fn call(&self, name: u64, state: &mut ProgramState) {
        // TODO: Verify procedures args
        state.ip = self.procedures.get(&name).unwrap().mem_start;
    }
}
