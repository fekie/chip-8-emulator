use crate::chip_8::{Chip8, Chip8Error};

// For the stack, the bottom of our stack if at 0x1FE (must be an even number
// if we want to increase the stack by 2 at a time), and the
// top is at 0x000. I've always wanted to actually implement
// a stack and I wanted to have it grow downwards for the
// true stack experience.
pub(crate) const STACK_WINDOW_BOTTOM: u16 = 0x1FE;
pub(crate) const STACK_WINDOW_TOP: u16 = 0x000;

impl Chip8 {
    pub(crate) fn push(&mut self, word: u16) -> Result<(), Chip8Error> {
        if self.stack_pointer == STACK_WINDOW_TOP {
            return Err(Chip8Error::StackOverflow);
        }

        self.stack_pointer -= 2;
        self.memory.set_word(self.stack_pointer as usize, word);

        Ok(())
    }

    pub(crate) fn pop(&mut self) -> Result<u16, Chip8Error> {
        if self.stack_pointer == STACK_WINDOW_BOTTOM {
            return Err(Chip8Error::StackUnderflow);
        }

        let word = self.memory.word(self.stack_pointer as usize);

        self.stack_pointer += 2;

        Ok(word)
    }
}
