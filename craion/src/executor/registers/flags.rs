use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u16 {
        const ZERO = 1 << 0;
        const CARRY = 1 << 1;
        const NEGATIVE = 1 << 2;
        const HALT = 1 << 15;
    }
}
