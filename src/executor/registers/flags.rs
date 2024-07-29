use bitfield_struct::bitfield;

#[bitfield(u16)]
pub struct Flags {
    #[bits(1)]
    pub zero: bool,
    #[bits(1)]
    pub carry: bool,
    #[bits(1)]
    pub negative: bool,
    #[bits(13)]
    reserve: u16,
}
