use anyhow::{anyhow, Result};

const STACK_SIZE: usize = 16;

pub struct Stack([u16; STACK_SIZE]);

impl Stack {
    pub fn new() -> Self {
        Self([0u16; STACK_SIZE])
    }

    pub fn pop(&mut self, sp: usize) -> Result<u16> {
        if sp >= STACK_SIZE {
            Err(anyhow!("wrong stack pointer: {}", sp))
        } else {
            Ok(self.0[sp])
        }
    }

    pub fn push(&mut self, sp: usize, value: u16) -> Result<()> {
        if sp >= STACK_SIZE {
            Err(anyhow!("wrong stack pointer: {}", sp))
        } else {
            self.0[sp] = value;
            Ok(())
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
