//! A module set aside for containing all of the methods on [`Chip8`] that emulate
//! the execution of each instruction.

use log::error;

use crate::{chip_8::Chip8Error, Chip8, HEIGHT, WIDTH};

impl Chip8 {
    pub fn instruction_clear(&mut self) {
        self.screen.clear();
    }

    pub fn instruction_return(&mut self) -> Result<(), Chip8Error> {
        self.program_counter = self.pop()?;
        Ok(())
    }

    pub fn instruction_jump(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    pub fn instruction_call(&mut self, nnn: u16) -> Result<(), Chip8Error> {
        self.push(self.program_counter)?;
        self.program_counter = nnn;
        Ok(())
    }

    pub fn instruction_skip_if_register_equals(&mut self, vx: u8, nn: u8) {
        if self.registers[vx as usize] == nn {
            self.program_counter += 2;
        }
    }

    pub fn instruction_skip_if_register_not_equals(&mut self, vx: u8, nn: u8) {
        if self.registers[vx as usize] != nn {
            self.program_counter += 2;
        }
    }

    pub fn instruction_skip_if_register_vx_equals_vy(&mut self, vx: u8, vy: u8) {
        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.program_counter += 2;
        }
    }

    pub fn instruction_set_immediate(&mut self, vx: u8, nn: u8) {
        self.registers[vx as usize] = nn;
    }

    pub fn instruction_add_immediate(&mut self, vx: u8, nn: u8) {
        let wrapped_sum = self.registers[vx as usize].wrapping_add(nn);

        let overflow_ocurred = self.registers[vx as usize].checked_add(nn).is_none();

        self.registers[vx as usize] = wrapped_sum;
        self.registers[0xF] = overflow_ocurred as u8;
    }

    pub fn instruction_copy(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] = self.registers[vy as usize]
    }

    pub fn instruction_bitwise_or(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] |= self.registers[vy as usize]
    }

    pub fn instruction_bitwise_and(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] &= self.registers[vy as usize]
    }

    pub fn instruction_bitwise_xor(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] ^= self.registers[vy as usize]
    }

    pub fn instruction_add(&mut self, vx: u8, vy: u8) {
        let wrapped_sum = self.registers[vx as usize].wrapping_add(self.registers[vy as usize]);

        let overflow_ocurred = self.registers[vx as usize]
            .checked_add(self.registers[vy as usize])
            .is_none();

        self.registers[vx as usize] = wrapped_sum;
        self.registers[0xF] = overflow_ocurred as u8;
    }

    pub fn instruction_subtract(&mut self, vx: u8, vy: u8) {
        let wrapped_sum = self.registers[vx as usize].wrapping_sub(self.registers[vy as usize]);

        let underflow_occurred = self.registers[vx as usize]
            .checked_sub(self.registers[vy as usize])
            .is_none();

        self.registers[vx as usize] = wrapped_sum;
        self.registers[0xF] = underflow_occurred as u8;
    }

    pub fn instruction_right_shift(&mut self, vx: u8) {
        let least_significant = self.registers[vx as usize] & 0b0000_0001;
        self.registers[0xF] = least_significant;
        self.registers[vx as usize] >>= 1;
    }

    pub fn instruction_set_vx_to_vy_minus_vx(&mut self, vx: u8, vy: u8) {
        let wrapped_sum = self.registers[vy as usize].wrapping_sub(self.registers[vx as usize]);

        let underflow_occured = self.registers[vy as usize]
            .checked_sub(self.registers[vx as usize])
            .is_none();

        self.registers[vx as usize] = wrapped_sum;
        self.registers[0xF] = underflow_occured as u8;
    }

    pub fn instruction_left_shift(&mut self, vx: u8) {
        let most_significant = self.registers[vx as usize] & 0b1000_0000;
        self.registers[0xF] = most_significant;
        self.registers[vx as usize] <<= 1;
    }

    pub fn instruction_skip_if_register_vx_not_equals_vy(&mut self, vx: u8, vy: u8) {
        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.program_counter += 2;
        }
    }

    pub fn instruction_set_index_register(&mut self, nnn: u16) {
        self.index_register = nnn;
    }
    pub fn instruction_jump_with_pc_offset(&mut self, nnn: u16) {
        self.program_counter = self.registers[0x0 as usize] as u16 + nnn;
    }
    pub fn instruction_random(&mut self, vx: u8, nn: u8) {
        self.registers[vx as usize] = rand::Rng::gen_range(&mut rand::thread_rng(), 0..=255) & nn
    }

    pub fn instruction_draw(&mut self, vx: u8, vy: u8, n: u8) {
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
        if let Some(frame_handle) = &self.frame_handle {
            frame_handle
                .send(Box::new(self.screen.get().clone()))
                .inspect_err(|e| error!("Error sending frame {e}"))
                .unwrap();
        }
    }

    pub fn instruction_skip_if_key_pressed(&mut self, vx: u8) {
        if let Some(keycode) = self.key_pressed {
            if keycode == self.registers[vx as usize] {
                self.program_counter += 2;
            }
        }
    }

    pub fn instruction_skip_if_key_not_pressed(&mut self, vx: u8) {
        if let Some(keycode) = self.key_pressed {
            if keycode != self.registers[vx as usize] {
                return;
            }
        }

        self.program_counter += 2;
    }

    pub fn instruction_set_vx_to_delay_timer(&mut self, vx: u8) {
        self.registers[vx as usize] = self.sound_timer.0
    }

    pub fn instruction_await_key_input(&mut self, vx: u8) {
        if self.key_pressed.is_none() {
            self.program_counter -= 2;
            return;
        }

        self.registers[vx as usize] = self.key_pressed.unwrap();
    }

    pub fn instruction_set_delay_timer(&mut self, vx: u8) {
        self.delay_timer.0 = self.registers[vx as usize]
    }

    pub fn instruction_set_sound_timer(&mut self, vx: u8) {
        self.sound_timer.0 = self.registers[vx as usize]
    }

    pub fn instruction_add_to_index(&mut self, vx: u8) {
        //Says to ignore overflow and not set the VF register
        self.index_register += self.registers[vx as usize] as u16
    }

    pub fn instruction_set_index_to_font_character(&mut self, vx: u8) {
        self.index_register = self.registers[vx as usize] as u16
    }

    pub fn instruction_set_index_to_binary_coded_vx(&mut self, vx: u8) {
        self.memory.set_byte(
            { self.index_register } as usize,
            self.registers[vx as usize] / 100,
        );
        self.memory.set_byte(
            { self.index_register + 1 } as usize,
            { self.registers[vx as usize] / 10 } % 10,
        );
        self.memory.set_byte({ self.index_register + 2 } as usize, {
            self.registers[vx as usize] % 10
        });
    }

    pub fn instruction_dump_registers(&mut self, vx: u8) {
        for i in 0x0..=vx {
            self.memory.set_byte(
                { self.index_register + i as u16 } as usize,
                self.registers[i as usize],
            );
        }
    }

    pub fn instruction_load_registers(&mut self, vx: u8) {
        for i in 0x0..=vx {
            self.registers[i as usize] = self
                .memory
                .byte({ self.index_register + i as u16 } as usize)
        }
    }

    pub fn instruction_unknown(&mut self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test_super {}
