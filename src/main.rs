use chip_8_emulator::Chip8;
use clap::Parser;

#[derive(clap::Parser, Debug)]
struct Args {
    /// Path to the ROM that will be loaded.
    #[arg(short, long)]
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let chip_8 = Chip8::new();

    let rom_bytes = std::fs::read(args.rom)?;

    Ok(())
}
