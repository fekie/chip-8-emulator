use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::ControlFlow,
};
use winit_input_helper::WinitInputHelper;

//fix the lib errors idk, somehow we need to
//layout based on the chip 8 tutorial blog
pub fn handle_keyboard_input(
    input: &WinitInputHelper,
    control_flow: &mut ControlFlow,
) -> Option<u8> {
    if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
        *control_flow = ControlFlow::Exit;
        //Doesn't matter what we return;
        return Some(0x0);
    }

    if input.key_pressed(VirtualKeyCode::Key1) {
        //println!("1 Pressed"); <-- Debug
        return Some(0x0);
    }

    if input.key_pressed(VirtualKeyCode::Key2) {
        return Some(0x1);
    }

    if input.key_pressed(VirtualKeyCode::Key3) {
        return Some(0x2);
    }
    if input.key_pressed(VirtualKeyCode::Key4) {
        return Some(0x3);
    }
    if input.key_pressed(VirtualKeyCode::Q) {
        return Some(0x4);
    }
    if input.key_pressed(VirtualKeyCode::W) {
        return Some(0x5);
    }
    if input.key_pressed(VirtualKeyCode::E) {
        return Some(0x6);
    }
    if input.key_pressed(VirtualKeyCode::R) {
        return Some(0x7);
    }
    if input.key_pressed(VirtualKeyCode::A) {
        return Some(0x8);
    }
    if input.key_pressed(VirtualKeyCode::S) {
        return Some(0x9);
    }
    if input.key_pressed(VirtualKeyCode::D) {
        return Some(0xA);
    }
    if input.key_pressed(VirtualKeyCode::F) {
        return Some(0xB);
    }
    if input.key_pressed(VirtualKeyCode::Z) {
        return Some(0xC);
    }
    if input.key_pressed(VirtualKeyCode::X) {
        return Some(0xD);
    }
    if input.key_pressed(VirtualKeyCode::C) {
        return Some(0xF);
    }
    return None;
}
