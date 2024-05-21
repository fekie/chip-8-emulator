use crate::chip_8::{Chip8, Chip8Error, EmulatorState};

use super::{screen::Screen, stack, DelayTimer, SoundTimer};

/// The address where our program starts in memory
pub(crate) const PROGRAM_OFFSET: usize = 0x200;
pub(crate) const FONT_SET_OFFSET: usize = 0x050;
pub(crate) const MEMORY_SIZE: usize = 0x1000;

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

/// Regions:
/// - 0x000-0x1FF is used for the CHIP-8 interpreter (used for the stack
/// in this implementation).
/// - 0x050-0x0A0 is used for the built-in pixel font set.
/// - 0x200-0xFFF is used for the program ROM and scratch RAM.
///
/// Has a capacity of [`MEMORY_SIZE`] bytes.
#[derive(Debug)]
pub(crate) struct Memory([u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Self([0; MEMORY_SIZE])
    }
}

impl Memory {
    /// Retrieves a byte from memory address.
    pub(crate) fn byte(&self, address: usize) -> u8 {
        self.0[address]
    }

    /// Sets a byte at memory address.
    pub(crate) fn set_byte(&mut self, address: usize, byte: u8) {
        self.0[address] = byte;
    }

    /// Retrieves a word from memory address. This combines
    /// `memory[address]` and `memory[address+1]` into a u16.
    pub(crate) fn word(&self, address: usize) -> u16 {
        ((self.0[address] as u16) << 8) | self.0[address + 1] as u16
    }

    #[allow(dead_code)]
    /// Sets a word at memory address. This writes to the
    /// bytes at `memory[address]` and `memory[address+1]`.
    pub(crate) fn set_word(&mut self, address: usize, word: u16) {
        self.0[address] = (word >> 8) as u8;
        self.0[address + 1] = (word & 0xFF) as u8
    }

    /// Loads the font set into the first 80 bytes of memory.
    pub(crate) fn load_font_set(&mut self) -> Result<(), Chip8Error> {
        // We load it in starting at where the program counter initializes to.
        let mut current_memory_address = FONT_SET_OFFSET;

        for byte in FONT_SET {
            self.set_byte(current_memory_address, byte);

            current_memory_address += 1;
        }

        Ok(())
    }
}

impl Chip8 {
    /// Initializes the emulator's system memory and loads fonts into memory.
    /// You can now load a program with [`Self::load_program`].
    pub fn initialize(&mut self) -> Result<(), Chip8Error> {
        // Clear memory
        self.memory = Memory::default();

        // Clear screen
        self.screen = Screen::default();

        self.registers = [0; 16];
        self.index_register = 0;
        self.program_counter = PROGRAM_OFFSET as u16;

        // Set the stack pointer to the value just under the stack, so that the
        // next push starts at bottom of the stack window.
        self.stack_pointer = stack::STACK_WINDOW_BOTTOM + 1;

        self.delay_timer = DelayTimer::default();
        self.sound_timer = SoundTimer::default();
        self.key_pressed = None;

        if let Some(frame_handle) = &self.frame_handle {
            frame_handle
                .send(Box::new(self.screen.get().clone()))
                .unwrap();
        }
        self.needs_program_restart = false;

        self.memory.load_font_set()?;

        self.emulator_state
            .change_states(EmulatorState::InterpreterMemoryInitialized)?;

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
            self.memory.set_byte(current_memory_address, byte);

            current_memory_address += 1;
        }

        // We clear out the rest of the bytes and variables as well so that
        // nothing interferes with this program (under the assumption that this
        // can be called multiple times to switch programs).
        for address in current_memory_address..MEMORY_SIZE {
            self.memory.set_byte(address, 0);
        }

        Ok(())
    }
}
