//#![deny(warnings)]
#![feature(allocator_api)]
#![feature(ptr_metadata)]
#![feature(set_ptr_value)]

pub mod decoder;
pub mod executor;
pub mod instruction;
pub mod instruction_helper;
pub mod memory;
pub mod section_manager;
