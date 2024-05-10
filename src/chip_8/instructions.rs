//! This module relates to opcode processing and formatting.

use super::Chip8Error;
use crate::{Chip8, HEIGHT, WIDTH};

/// A representation of all the CHIP-8 opcodes.
///
/// The names of the opcodes are unofficial and made by me. This means
/// that they could be named inaccurately or be able to find resources
/// on. Because of this, the hexadecimal representation is stated
/// in the docs above each variant, using the following
/// placeholder symbols to represent different meanings.  
///
/// The following information was taken from the
/// [wikipedia page](https://en.wikipedia.org/wiki/CHIP-8#Opcode_table).
///
/// - NNN: address
/// - NN: 8-bit constant
/// - N: 4-bit constant
/// - X and Y: 4-bit register identifier
/// - PC : Program Counter
/// - I : 16bit register (For memory address) (Similar to void pointer);
/// - VN: One of the 16 available variables. N may be 0 to F (hexadecimal);
#[derive(Debug)]
pub enum Instruction {
    /// Represented by 0NNN.
    ///
    /// This will remain unimplemented as it was used to pause
    /// the chip-8 interpreter and run hardware specific code,
    /// which was not used for most games.
    #[allow(dead_code)]
    CallMachineCodeRoutine,
    /// Represented by `00E0`.
    ///
    /// Clears the screen.
    Clear,
    /// Represented by `00EE`.
    ///
    /// Returns from subroutine by popping the new program
    /// counter from the stack.
    Return,
    /// Represented by `1NNN`.
    ///
    /// Sets program counter to NNN.
    #[allow(missing_docs)]
    Jump { nnn: u16 },
    /// Represented by `2NNN`.
    ///
    /// Calls a subroutine by setting the program counter
    /// to NNN and pushing the previous program counter to
    /// the stack.
    Call { nnn: u16 },
    /// Represented by 3XNN.
    ///
    /// Skips over the instruction if register VX == NN.
    SkipIfRegisterEquals { vx: u8, nn: u8 },
    /// Represented by 4XNN.
    ///
    /// Skips over the instruction if register VX != NN.
    SkipIfRegisterNotEquals { vx: u8, nn: u8 },
    /// Represented by 5XY0.
    ///
    /// Skips over the instruction if register VX == VY.
    SkipIfRegisterVxEqualsVy { vx: u8, vy: u8 },
    /// Represented by `6XNN`.
    ///
    /// Sets register VX to NN.
    #[allow(missing_docs)]
    SetImmediate { vx: u8, nn: u8 },
    /// Represented by `7XNN`.
    ///
    /// Adds the value NN to register VX.
    AddImmediate { vx: u8, nn: u8 },
    /// Represented by `8XY0`
    ///
    /// Copies register VY to VX.
    Copy { vx: u8, vy: u8 },
    /// Represented by `8XY1`
    BitwiseOr { vx: u8, vy: u8 },
    /// Represented by `8XY2`
    BitwiseAnd { vx: u8, vy: u8 },
    /// Represented by `8XY3`
    BitwiseXor { vx: u8, vy: u8 },
    /// Represented by `8XY4`
    Add { vx: u8, vy: u8 },
    /// Represented by `8XY5`
    Subtract { vx: u8, vy: u8 },
    /// Represented by `8XY6`
    RightShift { vx: u8 },
    /// Represented by `8XY7`
    SetVxToVyMinusVx { vx: u8, vy: u8 },
    /// Represented by `8XYE`
    LeftShift { vx: u8 },
    /// Represented by 9XY0.
    ///
    /// Skips over the instruction if register VX != VY.
    SkipIfRegisterVxNotEqualsVy { vx: u8, vy: u8 },
    /// Represented by `ANNN`.
    SetIndexRegister { nnn: u16 },
    /// Represented by `BNNN`.
    JumpWithPcOffset { nnn: u16 },
    /// Represented by `CXNN`.
    Random { vx: u8, nn: u8 },
    /// Represented by `DXYN`.
    Draw { vx: u8, vy: u8, n: u8 },
    /// Represented by `EX9E`.
    SkipIfKeyPressed { vx: u8 },
    /// Represented by `EXA1`.
    SkipIfKeyNotPressed { vx: u8 },
    /// Represented by `FX07`.
    SetVxToDelayTimer { vx: u8 },
    /// Represented by `FX0A`.
    AwaitKeyInput { vx: u8 },
    /// Represented by `FX15`.
    SetDelayTimer { vx: u8 },
    /// Represented by `FX18`.
    SetSoundTimer { vx: u8 },
    /// Represented by `FX1E`.
    AddToIndex { vx: u8 },
    /// Represented by `FX29`.
    SetIndexToFontCharacter { vx: u8 },
    /// Represented by `FX33`.
    SetIndexToBinaryCodedVx { vx: u8 },
    /// Represented by `FX55`.
    DumpRegisters { vx: u8 },
    /// Represented by `FX65`.
    LoadRegisters { vx: u8 },
    /// A value that does not represent any instruction.
    ///
    /// If a raw instruction parses into this, it is
    /// erroneous.
    Unknown,
}

impl Instruction {
    pub fn new(raw: u16) -> Result<Instruction, Chip8Error> {
        // We extract the first nibble of the raw u16,
        // which helps us create a match tree to figure out
        // which opcode a u16 is.
        let first_nibble = raw >> 12;

        //println!("{:04X}", first_nibble);
        //println!("0x{:04X}", raw);

        // These arguments are in the same location in memory each time,
        // so it's just cleaner to write them all up here.
        let vx = ((raw & 0x0F00) >> 8) as u8;
        let vy = ((raw & 0x00F0) >> 4) as u8;
        let nnn = raw & 0x0FFF;
        let nn = (raw & 0x00FF) as u8;
        let n = (raw & 0x000F) as u8;

        let instruction = match first_nibble {
            0x0 => {
                let last_byte = raw & 0x00FF;

                match last_byte {
                    0xE0 => Self::Clear,
                    0xEE => Self::Return,
                    // 0NNN is technically an instruction, but we do not
                    // want to implement it because it runs machine-specific
                    // instructions and is not compatible with every
                    // CHIP-8 machine.
                    _ => return Err(Chip8Error::ProgramNotCompatible),
                }
            }
            0x1 => Self::Jump { nnn },
            0x2 => Self::Call { nnn },
            0x3 => Self::SkipIfRegisterEquals { vx, nn },
            0x4 => Self::SkipIfRegisterNotEquals { vx, nn },
            0x5 => Self::SkipIfRegisterVxEqualsVy { vx, vy },
            0x6 => Self::SetImmediate { vx, nn },
            0x7 => Self::AddImmediate { vx, nn },
            0x8 => {
                let last_nibble = (raw & 0x000F) as u8;

                match last_nibble {
                    0x0 => Self::Copy { vx, vy },
                    0x1 => Self::BitwiseOr { vx, vy },
                    0x2 => Self::BitwiseAnd { vx, vy },
                    0x3 => Self::BitwiseXor { vx, vy },
                    0x4 => Self::Add { vx, vy },
                    0x5 => Self::Subtract { vx, vy },
                    0x6 => Self::RightShift { vx },
                    0x7 => Self::SetVxToVyMinusVx { vx, vy },
                    0xE => Self::LeftShift { vx },
                    _ => return Err(Chip8Error::InvalidInstruction { instruction: raw }),
                }
            }
            0x9 => Self::SkipIfRegisterVxNotEqualsVy { vx, vy },
            0xA => Self::SetIndexRegister { nnn },
            0xB => Self::JumpWithPcOffset { nnn },
            0xC => Self::Random { vx, nn },
            0xD => Self::Draw { vx, vy, n },
            0xE => {
                let last_byte = (raw & 0x00FF) as u8;

                match last_byte {
                    0x9E => Self::SkipIfKeyPressed { vx },
                    0xA1 => Self::SkipIfKeyNotPressed { vx },
                    _ => return Err(Chip8Error::InvalidInstruction { instruction: raw }),
                }
            }
            0xF => {
                let last_byte = (raw & 0x00FF) as u8;

                match last_byte {
                    0x07 => Self::SetVxToDelayTimer { vx },
                    0x0A => Self::AwaitKeyInput { vx },
                    0x15 => Self::SetDelayTimer { vx },
                    0x18 => Self::SetSoundTimer { vx },
                    0x1E => Self::AddToIndex { vx },
                    0x29 => Self::SetIndexToFontCharacter { vx },
                    0x33 => Self::SetIndexToBinaryCodedVx { vx },
                    0x55 => Self::DumpRegisters { vx },
                    0x65 => Self::LoadRegisters { vx },
                    _ => return Err(Chip8Error::InvalidInstruction { instruction: raw }),
                }
            }
            _ => return Err(Chip8Error::InvalidInstruction { instruction: raw }),
        };

        Ok(instruction)
    }
}

impl Chip8 {
    pub(crate) fn instruction_draw(&mut self, vx: u8, vy: u8, n: u8) {
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
