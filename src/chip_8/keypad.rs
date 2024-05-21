use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::ControlFlow,
};
use winit_input_helper::WinitInputHelper;

use super::Chip8Error;

//fix the lib errors idk, somehow we need to
//layout based on the chip 8 tutorial blog
pub fn handle_keyboard_input(
    input: &WinitInputHelper,
    control_flow: &mut ControlFlow,
) -> Result<Option<u8>, Chip8Error> {
    if input.key_held(VirtualKeyCode::Escape) || input.close_requested() {
        *control_flow = ControlFlow::Exit;
        //Doesn't matter what we return;
        return Ok(Some(0x0));
    }

    if input.key_held(VirtualKeyCode::Key1) {
        //println!("1 held"); <-- Debug
        return Ok(Some(0x0));
    }

    if input.key_held(VirtualKeyCode::Key2) {
        return Ok(Some(0x1));
    }

    if input.key_held(VirtualKeyCode::Key3) {
        return Ok(Some(0x2));
    }
    if input.key_held(VirtualKeyCode::Key4) {
        return Ok(Some(0x3));
    }
    if input.key_held(VirtualKeyCode::Q) {
        return Ok(Some(0x4));
    }
    if input.key_held(VirtualKeyCode::W) {
        return Ok(Some(0x5));
    }
    if input.key_held(VirtualKeyCode::E) {
        return Ok(Some(0x6));
    }
    if input.key_held(VirtualKeyCode::R) {
        return Ok(Some(0x7));
    }
    if input.key_held(VirtualKeyCode::A) {
        return Ok(Some(0x8));
    }
    if input.key_held(VirtualKeyCode::S) {
        return Ok(Some(0x9));
    }
    if input.key_held(VirtualKeyCode::D) {
        return Ok(Some(0xA));
    }
    if input.key_held(VirtualKeyCode::F) {
        return Ok(Some(0xB));
    }
    if input.key_held(VirtualKeyCode::Z) {
        return Ok(Some(0xC));
    }
    if input.key_held(VirtualKeyCode::X) {
        return Ok(Some(0xD));
    }
    if input.key_held(VirtualKeyCode::C) {
        return Ok(Some(0xF));
    }
    if input.key_held(VirtualKeyCode::Tab) {
        return Err(Chip8Error::ProgramRestartRequested);
    }
    return Ok(None);
}
