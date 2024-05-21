//! An implementation of an emulator for the CHIP-8 interpreter.

#![warn(missing_docs, missing_debug_implementations)]

use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use self::{instructions::Instruction, screen::Screen, sound::play_buzzer};
use instructions::execution;
use memory::Memory;

mod instructions;
pub(crate) mod keypad;
mod memory;
mod screen;
pub(crate) mod sound;
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
    #[error("Program Restart Requested")]
    ProgramRestartRequested,
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
#[derive(Debug, Default, Copy, Clone)]
pub struct DelayTimer(pub u8);

/// A timer that counts down at 60Hz. If above 0, the timer will be "active"
/// and count down to 0. At this point, a sound plays.  
#[derive(Debug, Default, Copy, Clone)]
pub struct SoundTimer(pub u8);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum EmulatorState {
    #[default]
    InterpreterMemoryUninitialized,
    InterpreterMemoryInitialized,
    ProgramLoaded,
}

impl EmulatorState {
    fn change_states(&mut self, new_state: EmulatorState) -> Result<(), Chip8Error> {
        if new_state == Self::InterpreterMemoryUninitialized {
            unreachable!("Cannot uninitialize uninitialized memory.")
        }

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
    pub delay_timer: DelayTimer,
    /// See [`SoundTimer`] for more information.
    pub sound_timer: SoundTimer,
    emulator_state: EmulatorState,
    /// The current key that is pressed down.
    pub key_pressed: Option<u8>,
    /// If this is true, then we need to redraw the frame.
    pub needs_redraw: bool,
    pub needs_program_restart: bool,
    frame_handle: Option<Sender<Box<[u8]>>>,
    input_handle: Option<Receiver<Result<Option<u8>, Chip8Error>>>,
}

impl Chip8 {
    /// Creates a new emulator with empty memory. You still have to initialize
    /// to with [`Self::initialize`] to load programs.
    pub fn new(
        frame_handle: Sender<Box<[u8]>>,
        input_handle: Receiver<Result<Option<u8>, Chip8Error>>,
    ) -> Self {
        Self {
            frame_handle: Some(frame_handle),
            input_handle: Some(input_handle),
            ..Default::default()
        }
    }

    pub fn print_all_registers(&self) {
        for i in 0x0..=0xF {
            print!("Register {i} is {}\n", self.registers[i as usize]);
        }
    }

    pub fn print_current_op(&self) {
        print!("{}\n", self.memory.word(self.index_register as usize));
    }
    /// Runs a moves the emulator state by one cycle. Requires both the interpreter memory
    /// to be initialized via [`Self::initialize`] and a program to be loaded in with
    /// [`Self::load_program`].
    pub fn cycle(&mut self) -> Result<(), Chip8Error> {
        if self.emulator_state != EmulatorState::ProgramLoaded {
            return Err(Chip8Error::ProgramNotLoaded);
        }
        if let Some(input_reciever) = &self.input_handle {
            self.key_pressed = match input_reciever.try_recv() {
                Ok(Ok(x)) => x,
                Ok(Err(e)) => match e {
                    Chip8Error::ProgramRestartRequested => {
                        self.initialize()?;
                        None
                    }
                    _ => panic!("{}", e),
                },
                Err(TryRecvError::Empty) => self.key_pressed,
                _ => panic!("Error receiving keypress."),
            }
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
            Instruction::Clear => self.instruction_clear(),
            Instruction::Return => self.instruction_return()?,
            Instruction::Jump { nnn } => self.instruction_jump(nnn),
            Instruction::Call { nnn } => self.instruction_call(nnn)?,
            Instruction::SkipIfRegisterEquals { vx, nn } => {
                self.instruction_skip_if_register_equals(vx, nn)
            }
            Instruction::SkipIfRegisterNotEquals { vx, nn } => {
                self.instruction_skip_if_register_not_equals(vx, nn)
            }
            Instruction::SkipIfRegisterVxEqualsVy { vx, vy } => {
                self.instruction_skip_if_register_vx_equals_vy(vx, vy)
            }
            Instruction::SetImmediate { vx, nn } => self.instruction_set_immediate(vx, nn),
            Instruction::AddImmediate { vx, nn } => self.instruction_add_immediate(vx, nn),
            Instruction::Copy { vx, vy } => self.instruction_copy(vx, vy),
            Instruction::BitwiseOr { vx, vy } => self.instruction_bitwise_or(vx, vy),
            Instruction::BitwiseAnd { vx, vy } => self.instruction_bitwise_and(vx, vy),
            Instruction::BitwiseXor { vx, vy } => self.instruction_bitwise_xor(vx, vy),
            Instruction::Add { vx, vy } => self.instruction_add(vx, vy),
            Instruction::Subtract { vx, vy } => self.instruction_subtract(vx, vy),
            Instruction::RightShift { vx } => self.instruction_right_shift(vx),
            Instruction::SetVxToVyMinusVx { vx, vy } => {
                self.instruction_set_vx_to_vy_minus_vx(vx, vy)
            }
            Instruction::LeftShift { vx } => self.instruction_left_shift(vx),
            Instruction::SkipIfRegisterVxNotEqualsVy { vx, vy } => {
                self.instruction_skip_if_register_vx_not_equals_vy(vx, vy)
            }
            Instruction::SetIndexRegister { nnn } => self.instruction_set_index_register(nnn),
            Instruction::JumpWithPcOffset { nnn } => self.instruction_jump_with_pc_offset(nnn),
            Instruction::Random { vx, nn } => self.instruction_random(vx, nn),
            Instruction::Draw { vx, vy, n } => self.instruction_draw(vx, vy, n),
            Instruction::SkipIfKeyPressed { vx } => self.instruction_skip_if_key_pressed(vx),
            Instruction::SkipIfKeyNotPressed { vx } => self.instruction_skip_if_key_not_pressed(vx),
            Instruction::SetVxToDelayTimer { vx } => self.instruction_set_vx_to_delay_timer(vx),
            Instruction::AwaitKeyInput { vx } => self.instruction_await_key_input(vx),
            Instruction::SetDelayTimer { vx } => self.instruction_set_delay_timer(vx),
            Instruction::SetSoundTimer { vx } => self.instruction_set_sound_timer(vx),
            Instruction::AddToIndex { vx } => self.instruction_add_to_index(vx),
            Instruction::SetIndexToFontCharacter { vx } => {
                self.instruction_set_index_to_font_character(vx)
            }
            Instruction::SetIndexToBinaryCodedVx { vx } => {
                self.instruction_set_index_to_binary_coded_vx(vx)
            }
            Instruction::DumpRegisters { vx } => self.instruction_dump_registers(vx),
            Instruction::LoadRegisters { vx } => self.instruction_load_registers(vx),
            Instruction::Unknown => self.instruction_unknown(),
        }

        Ok(())
    }
}
impl SoundTimer {
    pub fn decrement(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
            play_buzzer();
        }
    }
}
impl DelayTimer {
    pub fn decrement(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }
}
