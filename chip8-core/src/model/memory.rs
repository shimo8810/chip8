use anyhow::{anyhow, Result};

const MEMORY_SIZE: usize = 4096;

pub struct Memory([u8; MEMORY_SIZE]);

impl Memory {
    pub fn new() -> Self {
        Self([0u8; MEMORY_SIZE])
    }

    pub fn read(&self, addr: usize, size: usize) -> Result<&[u8]> {
        if addr + size > MEMORY_SIZE {
            Err(anyhow!("wrong memory address: {}..{}", addr, addr + size))
        } else {
            Ok(&self.0[addr..addr + size])
        }
    }

    pub fn store(&mut self, addr: usize, data: &[u8]) -> Result<()> {
        if addr + data.len() > MEMORY_SIZE {
            Err(anyhow!(
                "wrong memory address: {}..{}",
                addr,
                addr + data.len()
            ))
        } else {
            self.0[addr..addr + data.len()].copy_from_slice(data);
            Ok(())
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
