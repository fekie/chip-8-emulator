pub mod opcodes;

#[derive(Debug, thiserror::Error)]
pub enum Chip8Error {
    #[error("Error Parsing Opcode from String {0}")]
    ErrorParsingOpcodeFromString(String),
    #[error("Error Parsing Opcode From u16 {0}")]
    ErrorParsingOpcodeFromU16(String),
}

pub struct Chip8 {
    /// Regions:
    /// - 0x000-0x1FF is used for the CHIP-8 interpreter.
    /// - 0x050-0x0A0 is used for the built-in pixel font set.
    /// - 0x200-0xFFF is used for the program ROM and scratch RAM.
    memory: [u8; 0x1000],
    /// Starts with general purpose registers V0-VE. Fhe last register, VF
    // is used for the "carry" flag during addition, "no borrow" flag during
    /// subtraction, and is set upon pixel collision.
    registers: [u8; 0xF],
    /// We go with a 1024 byte stack, although this value here is somewhat arbitrary.
    stack: [u8; 0xFF],
    stack_pointer: u16,
    delay_timer: u8,
    sound_timer: u8,
    /// Acceptable values are 0-0xFFF.
    index_register: u16,
    /// Acceptable values are 0-0xFFF.
    sound_register: u16,
    // Represents the pixel states of a 64 x 32 screen.
    graphics_memory: [u8; 0x800],
    /// Stores the state of the hex keypad, which goes from 0x0 to 0xF.
    keypad: [u8; 0xF],
}
