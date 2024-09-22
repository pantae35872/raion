use common::register::RegisterType;
use proc::instruction;

use super::{InstructionArgument, InstructionError};

#[instruction(SAVR_OPCODE)]
pub fn savr(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);

    let reg = args.argument.parse_register()?;
    match reg {
        RegisterType::A64 | RegisterType::A32 | RegisterType::A16 | RegisterType::A8 => {
            args.register.save_a_register();
        }
        RegisterType::B64 | RegisterType::B32 | RegisterType::B16 | RegisterType::B8 => {
            args.register.save_b_register();
        }
        RegisterType::C64 | RegisterType::C32 | RegisterType::C16 | RegisterType::C8 => {
            args.register.save_c_register();
        }
        RegisterType::D64 | RegisterType::D32 | RegisterType::D16 | RegisterType::D8 => {
            args.register.save_d_register();
        }
        _ => return Err(InstructionError::SavedNonGeneral),
    }
    return Ok(());
}