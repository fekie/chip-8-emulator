//! An implementation of an emulator for the CHIP-8 interpreter.

#![warn(missing_docs, missing_debug_implementations)]

pub mod opcodes;

use bytebuffer::ByteBuffer;
use opcodes::Opcode;

const PROGRAM_COUNTER_INITIAL: usize = 0x200;

/// The default font set used in the CHIP-8 interpreter.
/// It works by treating the first 4 bits of each byte as pixels,
/// which means each subsequent byte translates to a row of pixels below
/// the current row.
///
/// This [website](https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
/// was used for the table, as well as a demonstration of how
/// this works.
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
/// - 0x050-0x0A0 is used for the built-in pixel font set (inside above range).
/// - 0x200-0xFFF is used for the program ROM and scratch RAM.
///
/// Has a capacity of 0x1000 bytes.
#[derive(Debug)]
pub struct Memory(ByteBuffer);

impl Default for Memory {
    fn default() -> Self {
        let mut bytebuffer = ByteBuffer::new();
        bytebuffer.resize(0x1000);
        Self(bytebuffer)
    }
}

/* impl Memory {
    fn load_font_set(&mut self) {
        self.0.read
    }
} */

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

// Acceptable values are 0-0xFFF.
#[derive(Debug, Default)]
pub struct ProgramCounter(usize);

/// Represents the pixel states of a 64 x 32 screen.
///
/// Has a capacity of 0x800 bytes.
#[derive(Debug)]
pub struct GraphicsMemory(ByteBuffer);

impl Default for GraphicsMemory {
    fn default() -> Self {
        let mut bytebuffer = ByteBuffer::new();
        bytebuffer.resize(0x800);
        Self(bytebuffer)
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
    program_counter: ProgramCounter,
    delay_timer: DelayTimer,
    sound_timer: SoundTimer,
    stack: Stack,
    stack_pointer: StackPointer,
    keypad: Keypad,
}

impl Chip8 {
    /// Creates a new emulator and initializes the memory in the emulator.
    pub fn new() -> Self {
        let program_counter = ProgramCounter(PROGRAM_COUNTER_INITIAL);

        let memory = Memory::default();

        todo!()
    }
}
