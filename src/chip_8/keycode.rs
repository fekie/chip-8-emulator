use minifb::{Key, Window};

use crate::Keycode;

use super::Chip8Error;

/// We use the following keypad mapping:
/// ```
/// Keypad                   Keyboard
/// +-+-+-+-+                +-+-+-+-+
/// |1|2|3|C|                |1|2|3|4|
/// +-+-+-+-+                +-+-+-+-+
/// |4|5|6|D|                |Q|W|E|R|
/// +-+-+-+-+       =>       +-+-+-+-+
/// |7|8|9|E|                |A|S|D|F|
/// +-+-+-+-+                +-+-+-+-+
/// |A|0|B|F|                |Z|X|C|V|
/// +-+-+-+-+                +-+-+-+-+
/// ```
pub fn get_available_keycode(window: &Window) -> Keycode {
    if window.is_key_down(Key::Key1) {
        return Keycode(Some(0x1));
    }

    if window.is_key_down(Key::Key2) {
        return Keycode(Some(0x2));
    }

    if window.is_key_down(Key::Key3) {
        return Keycode(Some(0x3));
    }

    if window.is_key_down(Key::Key4) {
        return Keycode(Some(0xC));
    }

    if window.is_key_down(Key::Q) {
        return Keycode(Some(0x4));
    }

    if window.is_key_down(Key::W) {
        return Keycode(Some(0x5));
    }

    if window.is_key_down(Key::E) {
        return Keycode(Some(0x6));
    }

    if window.is_key_down(Key::R) {
        return Keycode(Some(0xD));
    }

    if window.is_key_down(Key::A) {
        return Keycode(Some(0x7));
    }

    if window.is_key_down(Key::S) {
        return Keycode(Some(0x8));
    }

    if window.is_key_down(Key::D) {
        return Keycode(Some(0x9));
    }

    if window.is_key_down(Key::F) {
        return Keycode(Some(0xE));
    }

    if window.is_key_down(Key::Z) {
        return Keycode(Some(0xA));
    }

    if window.is_key_down(Key::X) {
        return Keycode(Some(0x0));
    }

    if window.is_key_down(Key::C) {
        return Keycode(Some(0xB));
    }

    if window.is_key_down(Key::V) {
        return Keycode(Some(0xF));
    }

    return Keycode(None);
}
