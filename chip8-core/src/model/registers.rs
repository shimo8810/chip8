use anyhow::{anyhow, Result};

const V_SIZE: usize = 16;

pub struct Registers {
    ///  general purpose registers (8-bit, x16)
    v: [u8; V_SIZE],
    /// memory adress register (16bit)
    pub i: u16,
    /// program counter (16-bit)
    pub pc: u16,
    /// stack pointer (8-bit)
    pub sp: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v: [0u8; V_SIZE],
            i: 0u16,
            pc: 0u16,
            sp: 0u8,
        }
    }

    pub fn set_v(&mut self, x: usize, value: u8) -> Result<()> {
        if x >= V_SIZE {
            Err(anyhow!("wrong register number: {}", x))
        } else {
            self.v[x] = value;
            Ok(())
        }
    }

    pub fn get_v(&mut self, x: usize) -> Result<u8> {
        if x >= V_SIZE {
            Err(anyhow!("wrong register number: {}", x))
        } else {
            Ok(self.v[x])
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
