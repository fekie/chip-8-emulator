//! A module set aside for containing all of the methods on [`Chip8`] that emulate
//! the execution of each instruction.

use crate::{Chip8, HEIGHT, WIDTH};

impl Chip8 {
    pub(crate) fn instruction_draw(&mut self, vx: u8, vy: u8, n: u8) {
        // Initialize VF
        self.registers[0xF] = 0;

        let mut x = self.registers[vx as usize] % WIDTH as u8;
        let mut y = self.registers[vy as usize] % HEIGHT as u8;

        for row in 0..n {
            let sprite_byte = self
                .memory
                .byte(self.index_register as usize + row as usize);

            // We iterate through the bits in the byte from left to right,
            // where each corresponds with an x value.
            for shift in (0..=7).rev() {
                let needs_invert = ((sprite_byte >> shift) & 0b0000_0001) == 1;

                // If we have a bit at this position, flip
                // the corresponding pixel. If we turned this
                // pixel off (and it used to be on), then
                // set VF to 1.
                if needs_invert {
                    let new_state = self.screen.invert(x, y);

                    if !new_state {
                        self.registers[0xF] = 1;
                    }
                }

                // Increment x
                x += 1;

                // End early if we are at the end of the screen.
                if x == WIDTH as u8 {
                    break;
                }
            }

            // Reset x to original value
            x = self.registers[vx as usize] % WIDTH as u8;

            // Increment y for every row
            y += 1;

            // End early if we are at the bottom of the screen.
            if y == HEIGHT as u8 {
                break;
            }
        }
    }
}
