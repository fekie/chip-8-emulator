//! An implementation of an emulator for the CHIP-8 interpreter.

#![warn(missing_docs, missing_debug_implementations)]

use self::{instruction::Instruction, screen::Screen};
use memory::Memory;

mod instruction;
mod memory;
mod screen;
mod stack;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

/// An error used for errors related to the operation of the CHIP-8 emulator.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Chip8Error {
    #[error("Interpreter memory is uninitialized")]
    InterpreterMemoryIsUninitialized,
    #[error("Interpreter memory already initialized")]
    InterpreterMemoryAlreadyInitialized,
    #[error("Program not loaded")]
    ProgramNotLoaded,
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
}

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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    /// Used for pointing to memory locations.
    index_register: u16,
    /// Points to the next instruction.
    program_counter: u16,
    /// Points to the top of the stack.
    stack_pointer: u16,
    delay_timer: DelayTimer,
    /// See [`SoundTimer`] for more information.
    sound_timer: SoundTimer,
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

    /// Runs a moves the emulator state by one cycle. Requires both the interpreter memory
    /// to be initialized via [`Self::initialize`] and a program to be loaded in with
    /// [`Self::load_program`].
    pub fn cycle(&mut self) -> Result<(), Chip8Error> {
        if self.emulator_state != EmulatorState::ProgramLoaded {
            return Err(Chip8Error::ProgramNotLoaded);
        }

        let raw = self.fetch();
        let instruction = self.decode(raw);
        self.execute(instruction);

        Ok(())
    }

    /// Fetches the current instruction word and increments the PC by 2.
    fn fetch(&mut self) -> u16 {
        let word = self.memory.word(self.program_counter as usize);

        // If we increment the PC before we pull an instruction from it,
        // we're gonna have problems.
        self.program_counter += 2;

        word
    }

    /// Decodes the instruction word into an [`Instruction`]
    fn decode(&self, raw: u16) -> Instruction {
        Instruction::new(raw)
    }

    /// Executes the provided instruction.
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
            Instruction::Draw { vx, vy, n } => self.instruction_draw(vx, vy, n),
            _ => unimplemented!(),
        }
    }

    fn instruction_draw(&mut self, vx: u8, vy: u8, n: u8) {
        // Initialize VF
        self.registers[0xF] = 0;

        let mut x = self.registers[vx as usize] % WIDTH as u8;
        let mut y = self.registers[vy as usize] % HEIGHT as u8;

        for row in 0..n {
            let sprite_byte = self
                .memory
                .byte(self.index_register as usize + row as usize);

            // We iterate through the bits in the byte from left to right,
            // where each corresponds with an x value.
            for shift in (0..=7).rev() {
                let needs_invert = ((sprite_byte >> shift) & 0b0000_0001) == 1;

                // If we have a bit at this position, flip
                // the corresponding pixel. If we turned this
                // pixel off (and it used to be on), then
                // set VF to 1.
                if needs_invert {
                    let new_state = self.screen.invert(x, y);

                    if !new_state {
                        self.registers[0xF] = 1;
                    }
                }

                // Increment x
                x += 1;

                // End early if we are at the end of the screen.
                if x == WIDTH as u8 {
                    break;
                }
            }

            // Reset x to original value
            x = self.registers[vx as usize] % WIDTH as u8;

            // Increment y for every row
            y += 1;

            // End early if we are at the bottom of the screen.
            if y == HEIGHT as u8 {
                break;
            }
        }
    }
}
