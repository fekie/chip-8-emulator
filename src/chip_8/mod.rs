//! An implementation of an emulator for the CHIP-8 interpreter.

#![warn(missing_docs, missing_debug_implementations)]

use self::{instruction::Instruction, screen::Screen};

mod instruction;
mod screen;

/// The address where our program starts in memory
const PROGRAM_OFFSET: usize = 0x200;
const FONT_SET_OFFSET: usize = 0x050;
const MEMORY_SIZE: usize = 0x1000;

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

/// An error used for errors related to the operation of the CHIP-8 emulator.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Chip8Error {
    #[error("Error parsing opcode from String {0}.")]
    ErrorParsingOpcodeFromString(String),
    #[error("Error parsing opcode from u16 {0}.")]
    ErrorParsingOpcodeFromU16(String),
    #[error("Not enough memory.")]
    NotEnoughMemory,
    #[error("Interpreter memory is uninitialized.")]
    InterpreterMemoryIsUninitialized,
    #[error("Interpreter memory already initialized.")]
    InterpreterMemoryAlreadyInitialized,
    #[error("Program not loaded.")]
    ProgramNotLoaded,
}

/// Regions:
/// - 0x000-0x1FF is used for the CHIP-8 interpreter (unused in this implementation).
/// - 0x050-0x0A0 is used for the built-in pixel font set.
/// - 0x200-0xFFF is used for the program ROM and scratch RAM.
///
/// Has a capacity of [`MEMORY_SIZE`] bytes.
#[derive(Debug)]
struct Memory([u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Self([0; MEMORY_SIZE])
    }
}

impl Memory {
    /// Loads the font set into the first 80 bytes of memory.
    fn load_font_set(&mut self) -> Result<(), Chip8Error> {
        // We load it in starting at where the program counter initializes to.
        let mut current_memory_address = FONT_SET_OFFSET;

        for byte in FONT_SET {
            match self.0.get_mut(current_memory_address) {
                Some(memory_byte) => *memory_byte = byte,
                None => return Err(Chip8Error::NotEnoughMemory),
            }

            current_memory_address += 1;
        }

        Ok(())
    }
}

/// We go with a 32 word stack
#[derive(Debug, Default)]
struct Stack([u16; 32]);

/// A timer that counts down at 60Hz. If above 0, the timer will be "active"
/// and count down to 0. At this point, a sound plays.  
#[derive(Debug, Default)]
struct DelayTimer(u8);

/// A timer that counts down at 60Hz. If above 0, the timer will be "active"
/// and count down to 0. At this point, a sound plays.  
#[derive(Debug, Default)]
struct SoundTimer(u8);

/// Stores the state of the hex keypad, which goes from 0x0 to 0xF.
#[derive(Debug, Default)]
struct Keypad([u8; 0xF]);

#[derive(Clone, Copy, Debug, Default)]
enum EmulatorState {
    #[default]
    InterpreterMemoryUninitialized,
    InterpreterMemoryInitialized,
    ProgramLoaded,
}

impl EmulatorState {
    fn change_states(&mut self, new_state: EmulatorState) -> Result<(), Chip8Error> {
        match new_state {
            // If it's moving to the initialized state, we just want to panic
            // because the user has definitely used some code that needs to be looked at.
            Self::InterpreterMemoryUninitialized => {
                panic!("Cannot uninitialize uninitialized memory.")
            }
            Self::InterpreterMemoryInitialized => match self {
                Self::InterpreterMemoryInitialized => {
                    return Err(Chip8Error::InterpreterMemoryAlreadyInitialized)
                }
                Self::ProgramLoaded => return Err(Chip8Error::InterpreterMemoryAlreadyInitialized),
                _ => {}
            },

            Self::ProgramLoaded => {
                if let Self::InterpreterMemoryUninitialized = self {
                    return Err(Chip8Error::InterpreterMemoryIsUninitialized);
                }
            }
        };

        // If we don't meet any invalid states, move to the next state.
        *self = new_state;
        Ok(())
    }
}

/// A struct used to emulate a CHIP-8 interpreter.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Chip8 {
    /// See [`Memory`] for more information.
    memory: Memory,
    /// See [`Screen`] for more information.
    screen: Screen,
    /// The registers used for emulating general purpose registers V0-VE.
    ///
    /// Starts with general purpose registers V0-VE. Fhe last register, VF
    // is used for the "carry" flag during addition, "no borrow" flag during
    /// subtraction, and is set upon pixel collision.
    registers: [u8; 16],
    /// See [`IndexRegister`] for more information.
    index_register: u16,
    program_counter: u16,
    delay_timer: DelayTimer,
    /// See [`SoundTimer`] for more information.
    sound_timer: SoundTimer,
    /// See [`Stack`] for more information.
    stack: Stack,
    /// See [`Keypad`] for more information.
    keypad: Keypad,
    emulator_state: EmulatorState,
}

impl Chip8 {
    /// Creates a new emulator with empty memory. You still have to initialize
    /// to with [`Self::initialize`] to load programs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the emulator's system memory and screen. You can now load a program
    /// with [`Self::load_program`].
    pub fn initialize(&mut self) -> Result<(), Chip8Error> {
        self.emulator_state
            .change_states(EmulatorState::InterpreterMemoryInitialized)?;

        self.program_counter = PROGRAM_OFFSET as u16;
        self.memory.load_font_set()?;

        // Screen memory is already initialized.
        // The actual screen window is initialized in the main function

        Ok(())
    }

    /// Loads a program into memory from raw bytes. Requires that [`Self::initialize`]
    /// has been called. You can now start emulation cycles with [`Self::cycle`].
    ///
    /// To load a new program, simply call [`Self::load_program`] again..
    pub fn load_program(&mut self, program_bytes: Vec<u8>) -> Result<(), Chip8Error> {
        self.emulator_state
            .change_states(EmulatorState::ProgramLoaded)?;

        // We load it in starting at the program offset.
        let mut current_memory_address = PROGRAM_OFFSET;

        for byte in program_bytes {
            match self.memory.0.get_mut(current_memory_address) {
                Some(memory_byte) => *memory_byte = byte,
                None => return Err(Chip8Error::NotEnoughMemory),
            }

            current_memory_address += 1;
        }

        // We clear out the rest of the bytes and variables as well so that
        // nothing interferes with this program (under the assumption that this
        // can be called multiple times to switch programs).
        for memory_address in current_memory_address..MEMORY_SIZE {
            self.memory.0[memory_address] = 0;
        }

        Ok(())
    }

    /// Runs a moves the emulator state by one cycle. Requires both the interpreter memory
    /// to be initialized via [`Self::initialize`] and a program to be loaded in with
    /// [`Self::load_program`].
    pub fn cycle(&mut self) -> Result<(), Chip8Error> {
        let raw = self.fetch();
        let instruction = self.decode(raw);
        self.execute(instruction);

        Ok(())
    }

    /// Fetches the current instruction word and increments the PC by 2.
    fn fetch(&mut self) -> u16 {
        let first_byte = self.memory.0[self.program_counter as usize];
        let second_byte = self.memory.0[self.program_counter as usize + 1];

        // If we increment the PC before we pull an instruction from it,
        // we're gonna have problems.
        self.program_counter += 2;

        ((first_byte as u16) << 8) | second_byte as u16
    }

    /// Decodes the instruction word into an [`Instruction`]
    fn decode(&self, raw: u16) -> Instruction {
        Instruction::new(raw)
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Clear => self.screen.clear(),
            Instruction::Jump { nnn } => {
                self.program_counter = nnn;
            }
            Instruction::SetRegister { vx, nn } => {
                self.registers[vx as usize] = nn;
            }
            Instruction::AddToRegisterVx { vx, nn } => {
                self.registers[vx as usize] += nn;
            }
            Instruction::SetIndexRegister { nnn } => {
                self.index_register = nnn;
            }
            Instruction::Draw { vx, vy, n } => {}
            _ => unimplemented!(),
        }
    }
}
