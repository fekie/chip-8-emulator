use chip_8_emulator::{opcodes::Opcode, Chip8};
use clap::Parser;

#[derive(clap::Parser, Debug)]
struct Args {
    /// Path to the ROM that will be loaded.
    #[arg(short, long)]
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut chip_8 = Chip8::new();
    chip_8.initialize()?;

    let program_bytes = std::fs::read(args.rom)?;

    chip_8.load_program(program_bytes.clone())?;

    let program_as_opcodes = program_bytes
        .iter()
        .step_by(2)
        .zip(program_bytes.iter().skip(1).step_by(2))
        .map(|(first_byte, next_byte)| {
            let combined = ((*first_byte as u16) << 8) | *next_byte as u16;
            Opcode::new(combined)
        })
        .collect::<Vec<Opcode>>();

    dbg!(program_as_opcodes);

    Ok(())
}
