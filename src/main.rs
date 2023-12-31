use std::fmt::{self, write};

#[derive(Debug, thiserror::Error)]
enum Chip8Error {
    #[error("Error Parsing Opcode {0}")]
    ErrorParsingOpcode(String),
}

/// A list of opcodes used by CHIP-8. These can be found
/// [here](https://en.wikipedia.org/wiki/CHIP-8#Opcode_table).
/// These are prefixed with a C- because these opcodes do not have official names,
/// and some of them start with numbers and enums cannot start with numbers.
enum Opcode {
    C0NNN,
    C00E0,
    C00EE,
    C1NNN,
    C2NNN,
    C3XNN,
    C4XNN,
    C5XY0,
    C6XNN,
    C7XNN,
    C8XY0,
    C8XY1,
    C8XY2,
    C8XY3,
    C8XY4,
    C8XY5,
    C8XY6,
    C8XY7,
    C8XYE,
    C9XY0,
    CANNN,
    CBNNN,
    CCXNN,
    CDXYN,
    CEX9E,
    CEXA1,
    CFX07,
    CFX0A,
    CFX15,
    CFX18,
    CFX1E,
    CFX29,
    CFX33,
    CFX55,
    CFX65,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::C0NNN => writeln!(f, "0NNN"),
            Self::C00E0 => writeln!(f, "00E0"),
            Self::C00EE => writeln!(f, "00EE"),
            Self::C1NNN => writeln!(f, "1NNN"),
            Self::C2NNN => writeln!(f, "2NNN"),
            Self::C3XNN => writeln!(f, "3XNN"),
            Self::C4XNN => writeln!(f, "4XNN"),
            Self::C5XY0 => writeln!(f, "5XY0"),
            Self::C6XNN => writeln!(f, "6XNN"),
            Self::C7XNN => writeln!(f, "7XNN"),
            Self::C8XY0 => writeln!(f, "8XY0"),
            Self::C8XY1 => writeln!(f, "8XY1"),
            Self::C8XY2 => writeln!(f, "8XY2"),
            Self::C8XY3 => writeln!(f, "8XY3"),
            Self::C8XY4 => writeln!(f, "8XY4"),
            Self::C8XY5 => writeln!(f, "8XY5"),
            Self::C8XY6 => writeln!(f, "8XY6"),
            Self::C8XY7 => writeln!(f, "8XY7"),
            Self::C8XYE => writeln!(f, "8XYE"),
            Self::C9XY0 => writeln!(f, "9XY0"),
            Self::CANNN => writeln!(f, "ANNN"),
            Self::CBNNN => writeln!(f, "BNNN"),
            Self::CCXNN => writeln!(f, "CXNN"),
            Self::CDXYN => writeln!(f, "DXYN"),
            Self::CEX9E => writeln!(f, "EX9E"),
            Self::CEXA1 => writeln!(f, "EXA1"),
            Self::CFX07 => writeln!(f, "FX07"),
            Self::CFX0A => writeln!(f, "FX0A"),
            Self::CFX15 => writeln!(f, "FX15"),
            Self::CFX18 => writeln!(f, "FX18"),
            Self::CFX1E => writeln!(f, "FX1E"),
            Self::CFX29 => writeln!(f, "FX29"),
            Self::CFX33 => writeln!(f, "FX33"),
            Self::CFX55 => writeln!(f, "FX55"),
            Self::CFX65 => writeln!(f, "FX65"),
        }
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl Opcode {
    pub fn try_new(raw: &str) -> Result<Self, Chip8Error> {
        match raw {
            "0NNN" => Ok(Self::C0NNN),
            "00E0" => Ok(Self::C00E0),
            "00EE" => Ok(Self::C00EE),
            "1NNN" => Ok(Self::C1NNN),
            "2NNN" => Ok(Self::C2NNN),
            "3XNN" => Ok(Self::C3XNN),
            "4XNN" => Ok(Self::C4XNN),
            "5XY0" => Ok(Self::C5XY0),
            "6XNN" => Ok(Self::C6XNN),
            "7XNN" => Ok(Self::C7XNN),
            "8XY0" => Ok(Self::C8XY0),
            "8XY1" => Ok(Self::C8XY1),
            "8XY2" => Ok(Self::C8XY2),
            "8XY3" => Ok(Self::C8XY3),
            "8XY4" => Ok(Self::C8XY4),
            "8XY5" => Ok(Self::C8XY5),
            "8XY6" => Ok(Self::C8XY6),
            "8XY7" => Ok(Self::C8XY7),
            "8XYE" => Ok(Self::C8XYE),
            "9XY0" => Ok(Self::C9XY0),
            "ANNN" => Ok(Self::CANNN),
            "BNNN" => Ok(Self::CBNNN),
            "CXNN" => Ok(Self::CCXNN),
            "DXYN" => Ok(Self::CDXYN),
            "EX9E" => Ok(Self::CEX9E),
            "EXA1" => Ok(Self::CEXA1),
            "FX07" => Ok(Self::CFX07),
            "FX0A" => Ok(Self::CFX0A),
            "FX15" => Ok(Self::CFX15),
            "FX18" => Ok(Self::CFX18),
            "FX1E" => Ok(Self::CFX1E),
            "FX29" => Ok(Self::CFX29),
            "FX33" => Ok(Self::CFX33),
            "FX55" => Ok(Self::CFX55),
            "FX65" => Ok(Self::CFX65),
            _ => Err(Chip8Error::ErrorParsingOpcode(raw.to_string())),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let foo = Opcode::try_new("FX55")?;

    dbg!(foo);

    Ok(())
}
