use crate::chip_8::Chip8Error;

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
