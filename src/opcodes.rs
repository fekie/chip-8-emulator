//! This module relates to opcode processing and formatting.

use super::chip_8::{Chip8, Chip8Error};
use std::fmt;

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
pub enum Opcode {
    /* /// Represented by 0NNN.
    ///
    /// This will likely remain unimplemented
    /// as the wikipedia page says it is not used in most roms, and it appears
    /// to possibly be hardware dependant.
    CallMachineCodeRoutine,
    /// Represented by `00E0`.
    ClearScreen,
    /// Represented by `00EE`.
    Return,
    /// Represented by `1NNN`.
    Jump,
    /// Represented by `00E0`.
    Call,
    /// Represented by `00E0`.
    SkipIfEqualsConstant,
    /// Represented by `00E0`.
    SkipIfNotEqual,
    /// Represented by `00E0`.
    SkipIfEqual,
    /// Represented by `00E0`.
    SetToConstant,
    /// Represented by `00E0`.
    AddConstant,
    /// Represented by `00E0`.
    SetToValue, */
    /// Represented by `00E0`.
    Clear,
    /// Represented by `00EE`.
    Return,
    /// Represented by `1NNN`.
    #[allow(missing_docs)]
    Jump { nnn: u16 },
    /// Represented by `2NNN`.
    ///
    /// Calls the subroutine at NNN.
    #[allow(missing_docs)]
    Call { nnn: u16 },
    /// Represented by 3XNN.
    ///
    /// Skips over the instruction if register VX == NN.
    #[allow(missing_docs)]
    SkipIfRegisterEquals { vx: u8, nn: u8 },
    /// Represented by 3XNN.
    ///
    /// Skips over the instruction if register VX != NN.
    #[allow(missing_docs)]
    SkipIfRegisterNotEquals { vx: u8, nn: u8 },
    /// Represented by 3XNN.
    ///
    /// Skips over the instruction if register VX == VY.
    #[allow(missing_docs)]
    SkipIfRegisterVxEqualsVy { vx: u8, vy: u8 },
    /// Represented by `6XNN`.
    ///
    /// Sets register VX to NN.
    #[allow(missing_docs)]
    SetRegister { vx: u8, nn: u8 },
    /// Represented by `7XNN`.
    AddToRegisterVx,
    /// Represented by `ANNN`.
    SetIndexRegister,
    /// Represented by `DXYN`.
    Draw,
    /// Represented by `FFFF`.
    ///
    /// Does not appear to do anything, but we still see it in
    /// programs.
    Unknown,
}

impl Opcode {
    pub fn new(raw: u16) -> Opcode {
        // We extract the first nibble of the raw u16,
        // which helps us create a match tree to figure out
        // which opcode a u16 is.
        let first_nibble = raw >> 12;

        println!("{:04X}", first_nibble);

        match first_nibble {
            0x0 => {
                let last_byte = raw & 0x00FF;

                match last_byte {
                    0xE0 => Self::Clear,
                    0xEE => Self::Return,
                    // 0NNN is technically an instruction, but we do not
                    // want to implement it because it runs machine-specific
                    // instructions and is not compatible with every
                    // CHIP-8 machine.
                    _ => unimplemented!(),
                }
            }
            0x1 => Self::Jump { nnn: raw & 0x0FFF },
            0x2 => Self::Call { nnn: raw & 0x0FFF },
            0x3 => Self::SkipIfRegisterEquals {
                vx: ((raw & 0x0F00) >> 8) as u8,
                nn: (raw & 0x00FF) as u8,
            },
            0x4 => Self::SkipIfRegisterNotEquals {
                vx: ((raw & 0x0F00) >> 8) as u8,
                nn: (raw & 0x00FF) as u8,
            },
            0x5 => Self::SkipIfRegisterVxEqualsVy {
                vx: ((raw & 0x0F00) >> 8) as u8,
                vy: ((raw & 0x00F0) >> 4) as u8,
            },
            0x6 => Self::SetRegister {
                vx: ((raw & 0x0F00) >> 8) as u8,
                nn: (raw & 0x00FF) as u8,
            },
            0x7 => Self::AddToRegisterVx,
            0xA => Self::SetIndexRegister,
            0xD => Self::Draw,
            _ => unimplemented!(),
        }
    }
}
