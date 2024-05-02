use crate::Chip8;
use crate::HEIGHT;
use crate::WIDTH;

/// The memory used for the screen. Each value is
/// a boolean and represents a 1 for white, and 0 for black.
///
/// The 0th memory location maps to the top left corner
/// of the screen.
/// A memory location is given by `location = WIDTH*y + x`.
#[derive(Debug)]
pub struct Screen([u8; (WIDTH * HEIGHT) as usize]);

impl Default for Screen {
    /// Initializes screen to black.
    fn default() -> Self {
        Self([0; (WIDTH * HEIGHT) as usize])
    }
}

impl Screen {
    /// Clears the screen.
    pub fn clear(&mut self) {
        for b in self.0.iter_mut() {
            *b = 0x00;
        }
    }
}

impl Chip8 {
    /// Draws the contents of the screen memory to the screen.
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = match self.screen.0[i] {
                0 => [0, 0, 0, 0xFF],
                1 => [0xFF, 0xFF, 0xFF, 0xFF],
                _ => panic!("Invalid screen memory value."),
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
