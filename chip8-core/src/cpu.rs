use crate::display::Display;
use crate::instruction::Instruction;

const V_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const MEM_SIZE: usize = 4096;

pub struct Cpu<D: Display> {
    /// general purpose v (8-bit, x16)
    pub v: [u8; V_SIZE],
    /// memory address register (16-bit)
    pub i: u16,
    /// program counter (16-bit)
    pub pc: u16,
    /// stack pointer (8-bit)
    pub sp: u8,
    /// stack (16-bit, x16)
    pub stack: [u16; STACK_SIZE],
    /// memory
    pub mem: [u8; MEM_SIZE],
    /// display
    pub dsp: D,
}

impl<D> Cpu<D>
where
    D: Display,
{
    pub fn new(display: D) -> Self {
        Self {
            v: [0; V_SIZE],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            mem: [0; MEM_SIZE],
            dsp: display,
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch();
        let instruction = self.decode(opcode);
        self.execute(instruction);
    }

    fn fetch(&self) -> u16 {
        let i = self.pc as usize;
        ((self.mem[i] as u16) << 8) | (self.mem[i + 1] as u16)
    }

    fn decode(&self, opcode: u16) -> Instruction {
        let addr = opcode & 0x0FFF;
        let byte = (opcode & 0x00FF) as u8;
        let nibble = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        // decode and execute
        match (
            ((opcode & 0xF000) >> 12),
            ((opcode & 0x0F00) >> 8),
            ((opcode & 0x00F0) >> 4),
            (opcode & 0x000F),
        ) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::Cls,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Ret,
            (0x1, _, _, _) => Instruction::JpAddr(addr),
            (0x2, _, _, _) => Instruction::CallAddr(addr),
            (0x3, _, _, _) => Instruction::SeVxByte(x, byte),
            (0x4, _, _, _) => Instruction::SneVxByte(x, byte),
            (0x5, _, _, 0x0) => Instruction::SeVxVy(x, y),
            (0x6, _, _, _) => Instruction::LdVxByte(x, byte),
            (0x7, _, _, _) => Instruction::AddVxByte(x, byte),
            (0x8, _, _, 0x0) => Instruction::LdVxVy(x, y),
            (0x8, _, _, 0x1) => Instruction::OrVxVy(x, y),
            (0x8, _, _, 0x2) => Instruction::AndVxVy(x, y),
            (0x8, _, _, 0x3) => Instruction::XorVxVy(x, y),
            (0x8, _, _, 0x4) => Instruction::AddVxVy(x, y),
            (0x8, _, _, 0x5) => Instruction::SubVxVy(x, y),
            (0x8, _, _, 0x6) => Instruction::ShrVx(x),
            (0x8, _, _, 0x7) => Instruction::SubnVxVy(x, y),
            (0x8, _, _, 0xE) => Instruction::ShlVx(x),
            (0x9, _, _, 0x0) => Instruction::SneVxVy(x, y),
            (0xA, _, _, _) => Instruction::LdIAddr(addr),
            (0xB, _, _, _) => Instruction::JpV0Addr(addr),
            (0xC, _, _, _) => Instruction::RndVxByte(x, byte),
            (0xD, _, _, _) => Instruction::DrwVxVyNibble(x, y, nibble),
            (0xE, _, 0x9, 0xE) => Instruction::SkpVx(x),
            (0xE, _, 0xA, 0x1) => Instruction::SkpnVx(x),
            (0xF, _, 0xA, 0x7) => Instruction::LdVxDt(x),
            (0xF, _, 0x0, 0xA) => Instruction::LdVxK(x),
            (0xF, _, 0x1, 0x5) => Instruction::LdDtVx(x),
            (0xF, _, 0x1, 0x8) => Instruction::LdStVx(x),
            (0xF, _, 0x1, 0xE) => Instruction::AddIVx(x),
            (0xF, _, 0x2, 0x9) => Instruction::LdFVx(x),
            (0xF, _, 0x3, 0x3) => Instruction::LdBVx(x),
            (0xF, _, 0x5, 0x5) => Instruction::LdIVx(x),
            (0xF, _, 0x6, 0x5) => Instruction::LdVxI(x),
            _ => Instruction::NOOP,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        //
        self.pc += 2;

        match instruction {
            Instruction::Cls => self.dsp.cls(),
            Instruction::Ret => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            Instruction::JpAddr(addr) => self.pc = addr,
            Instruction::CallAddr(addr) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            Instruction::SeVxByte(x, byte) => self.pc += if self.v[x] == byte { 2 } else { 0 },
            Instruction::SneVxByte(x, byte) => self.pc += if self.v[x] != byte { 2 } else { 0 },
            Instruction::SeVxVy(x, y) => self.pc += if self.v[x] == self.v[y] { 2 } else { 0 },
            Instruction::LdVxByte(x, byte) => self.v[x] = byte,
            Instruction::AddVxByte(x, byte) => self.v[x] += byte,
            Instruction::LdVxVy(x, y) => self.v[x] = self.v[y],
            Instruction::OrVxVy(x, y) => self.v[x] |= self.v[y],
            Instruction::AndVxVy(x, y) => self.v[x] &= self.v[y],
            Instruction::XorVxVy(x, y) => self.v[x] ^= self.v[y],
            Instruction::AddVxVy(x, y) => {
                let (value, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = value;
                self.v[0xF] = if carry { 1 } else { 0 };
            }
            Instruction::SubVxVy(x, y) => {
                let (value, carry) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = value;
                self.v[0xF] = if carry { 0 } else { 1 };
            }
            Instruction::ShrVx(x) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            }
            Instruction::SubnVxVy(x, y) => {
                let (value, carry) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = value;
                self.v[0xF] = if carry { 0 } else { 1 };
            }
            Instruction::ShlVx(x) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            }
            Instruction::SneVxVy(x, y) => self.pc += if self.v[x] != self.v[y] { 2 } else { 0 },
            Instruction::LdIAddr(addr) => self.i = addr,
            Instruction::JpV0Addr(addr) => self.pc = addr + self.v[0] as u16,
            Instruction::RndVxByte(x, byte) => {}
            Instruction::DrwVxVyNibble(x, y, nibble) => {
                let i = self.i as usize;
                let n = nibble as usize;
                self.v[0xF] = self.dsp.draw(x, y, &self.mem[i..i + n as usize]);
            }
            Instruction::SkpVx(x) => {}
            Instruction::SkpnVx(x) => {}
            Instruction::LdVxDt(x) => {}
            Instruction::LdVxK(x) => {}
            Instruction::LdDtVx(x) => {}
            Instruction::LdStVx(x) => {}
            Instruction::AddIVx(x) => self.i += self.v[x] as u16,
            Instruction::LdFVx(x) => {}
            Instruction::LdBVx(x) => {}
            Instruction::LdIVx(x) => {
                let i = self.i as usize;
                self.mem[i..=i + x].copy_from_slice(&self.v[0..=x]);
            }
            Instruction::LdVxI(x) => {
                let i = self.i as usize;
                self.v[0..=x].copy_from_slice(&self.mem[i..=i + x]);
            }
            Instruction::NOOP => {}
        }
    }
}
