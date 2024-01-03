//! This module relates to opcode processing and formatting.

use crate::{Chip8, Chip8Error};
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
    /// Represented by 0NNN. This will likely remain unimplemented
    /// as the wikipedia page says it is not used in most roms, and it appears
    /// to possibly be hardware dependant.
    Call,
    /// Represented of `00E0`.
    ClearScreen,
    /// Represented of `00EE`.
    Return,
    /// Represented of `00E0`.
    Jump,
}
