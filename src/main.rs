use chip_8_emulator::opcodes::Opcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let foo = Opcode::try_from_string("FX55")?;

    dbg!(foo);

    Ok(())
}
