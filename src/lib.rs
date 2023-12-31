//! An implementation of an emulator for the CHIP-8 interpreter.

#![warn(missing_docs, missing_debug_implementations)]

pub mod opcodes;

use opcodes::Opcode;

/// The error used for errors related to the operation of the CHIP-8 emulator.
#[derive(Debug, thiserror::Error)]
pub enum Chip8Error {
    #[error("Error Parsing Opcode from String {0}")]
    ErrorParsingOpcodeFromString(String),
    #[error("Error Parsing Opcode From u16 {0}")]
    ErrorParsingOpcodeFromU16(String),
}

/// Regions:
/// - 0x000-0x1FF is used for the CHIP-8 interpreter.
/// - 0x050-0x0A0 is used for the built-in pixel font set.
/// - 0x200-0xFFF is used for the program ROM and scratch RAM.
#[derive(Debug)]
pub struct Memory([u8; 0x1000]);

impl Default for Memory {
    fn default() -> Self {
        Self([0; 0x1000])
    }
}

/// Starts with general purpose registers V0-VE. Fhe last register, VF
// is used for the "carry" flag during addition, "no borrow" flag during
/// subtraction, and is set upon pixel collision.
#[derive(Debug, Default)]
pub struct Registers([u8; 0xF]);

/// We go with a 32 byte stack, allowing for a 16 level stack.
#[derive(Debug, Default)]
pub struct Stack([u16; 0xF]);

/// A pointer that points to the level of the stack we are using.
#[derive(Debug, Default)]
pub struct StackPointer(usize);

/// A timer that counts down at 60Hz. If above 0, the timer will be "active"
/// and count down to 0. At this point, a sound plays.  
#[derive(Debug, Default)]
pub struct DelayTimer(u8);

/// A timer that counts down at 60Hz. If above 0, the timer will be "active"
/// and count down to 0. At this point, a sound plays.  
#[derive(Debug, Default)]
pub struct SoundTimer(u8);

// Acceptable values are 0-0xFFF.
#[derive(Debug, Default)]
pub struct IndexRegister(u16);

/// Represents the pixel states of a 64 x 32 screen.
#[derive(Debug)]
pub struct GraphicsMemory([u8; 0x800]);

impl Default for GraphicsMemory {
    fn default() -> Self {
        Self([0; 0x800])
    }
}

/// Stores the state of the hex keypad, which goes from 0x0 to 0xF.
#[derive(Debug, Default)]
pub struct Keypad([u8; 0xF]);

#[allow(missing_docs, dead_code)]
#[derive(Debug, Default)]
pub struct Chip8 {
    memory: Memory,
    graphics_memory: GraphicsMemory,
    registers: Registers,
    index_register: IndexRegister,
    delay_timer: DelayTimer,
    sound_timer: SoundTimer,
    stack: Stack,
    stack_pointer: StackPointer,
    keypad: Keypad,
}

impl Chip8 {
    /// Creates a new [`Chip8`]. This pre-initializes the memory
    /// to where it is ready for a program to be loaded.
    pub fn new() -> Self {
        Self::default()
    }
}
