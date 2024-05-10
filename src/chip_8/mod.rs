//! An implementation of an emulator for the CHIP-8 interpreter.

#![warn(missing_docs, missing_debug_implementations)]

use self::{instructions::Instruction, screen::Screen};
use memory::Memory;

mod instructions;
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
    /// Triggered when the emulator encounters instruction 0NNN.
    /// This would normally pause the chip-8 interpreter and run
    /// hardware-dependant code, and is not used for the majority of roms.
    #[error("Program not compatible")]
    ProgramNotCompatible,
    /// Used when the raw word does not translate to an instruction,
    /// like 0xFFFF.
    #[error("Invalid Instruction 0x{instruction:04X}")]
    InvalidInstruction { instruction: u16 },
    /// Used when the execution code for an instruction is unimplemented.
    #[error("Unimplemented instruction {instruction:#?}")]
    UnimplementedInstruction { instruction: Instruction },
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
        let instruction = self.decode(raw)?;
        self.execute(instruction)?;

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
    fn decode(&self, raw: u16) -> Result<Instruction, Chip8Error> {
        Instruction::new(raw)
    }

    /// Executes the provided instruction.
    fn execute(&mut self, instruction: Instruction) -> Result<(), Chip8Error> {
        match instruction {
            Instruction::CallMachineCodeRoutine => {
                return Err(Chip8Error::UnimplementedInstruction { instruction })
            }
            Instruction::Clear => self.screen.clear(),

            Instruction::Jump { nnn } => {
                self.program_counter = nnn;
            }
            Instruction::SetImmediate { vx, nn } => {
                self.registers[vx as usize] = nn;
            }
            Instruction::AddImmediate { vx, nn } => {
                self.registers[vx as usize] += nn;
            }
            Instruction::SetIndexRegister { nnn } => {
                self.index_register = nnn;
            }
            Instruction::Copy { vx, vy } => {
                self.registers[vx as usize] = self.registers[vy as usize]
            }
            Instruction::Draw { vx, vy, n } => self.instruction_draw(vx, vy, n),
            Instruction::BitwiseOr { vx, vy } => {
                self.registers[vx as usize] |= self.registers[vy as usize]
            }
            Instruction::BitwiseAnd { vx, vy } => {
                self.registers[vx as usize] &= self.registers[vy as usize]
            }
            Instruction::BitwiseXor { vx, vy } => {
                self.registers[vx as usize] ^= self.registers[vy as usize]
            }
            Instruction::Add { vx, vy } => {
                let wrapped_sum =
                    self.registers[vx as usize].wrapping_add(self.registers[vy as usize]);

                let overflow_ocurred = self.registers[vx as usize]
                    .checked_add(self.registers[vy as usize])
                    .is_none();

                self.registers[vx as usize] = wrapped_sum;
                self.registers[0xF] = overflow_ocurred as u8;
            }
            Instruction::Subtract { vx, vy } => {
                let wrapped_sum =
                    self.registers[vx as usize].wrapping_sub(self.registers[vy as usize]);

                let underflow_occurred = self.registers[vx as usize]
                    .checked_sub(self.registers[vy as usize])
                    .is_none();

                self.registers[vx as usize] = wrapped_sum;
                self.registers[0xF] = underflow_occurred as u8;
            }
            Instruction::RightShift { vx } => {
                let least_significant = self.registers[vx as usize] & 0b0000_0001;
                self.registers[0xF] = least_significant;
                self.registers[vx as usize] >>= 1;
            }
            Instruction::LeftShift { vx } => {
                let most_significant = self.registers[vx as usize] & 0b1000_0000;
                self.registers[0xF] = most_significant;
                self.registers[vx as usize] <<= 1;
            }
            Instruction::Random { vx, nn } => {
                self.registers[vx as usize] = rand::Rng::gen_range(&mut rand::thread_rng(), 0..255)
                    & self.registers[nn as usize]
            }

            _ => return Err(Chip8Error::UnimplementedInstruction { instruction }),
        }

        Ok(())
    }
}
