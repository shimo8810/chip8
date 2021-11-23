#[derive(Debug)]
struct Chip8 {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Chip8 {
    fn new() -> Self {
        Chip8 {
            registers: [0u8; 16],
            program_counter: 0,
            memory: [0u8; 4096],
            stack: [0u16; 16],
            stack_pointer: 0,
        }
    }

    fn run(&mut self) {
        //
        let opcode = 0x8124u16;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        match opcode {
            0x0000 => {
                return;
            }
            0x1000..=0x1FFF => self.jump_to(nnn),
            0x2000..=0x2FFF => self.call_subroutine_at(nnn),
            0x3000..=0x3FFF => self.skip_if_vx_equals_nn(x, nn),
            0x4000..=0x4FFF => self.skip_if_vx_not_equals_nn(x, nn),
            0x5000..=0x5FFF => self.skip_if_vx_equals_vy(x, y),
            0x8000..=0x8FFF => match n {
                0x0 => self.set_vx_to_vy(x, y),
                0x1 => self.set_vx_to_vx_or_vy(x, y),
                0x2 => self.set_vx_to_vx_and_vy(x, y),
                0x3 => self.set_vx_to_vx_xor_vy(x, y),
                0x4 => self.add_vy_to_vx(x, y),
                0x5 => self.sub_vy_from_vx(x, y),
                0x6 => todo!("Vx >>= 1 0x{:04x}", opcode),
                0x7 => todo!("Vx = Vy - Vx 0x{:04x}", opcode),
                0xE => todo!("Vx <<= 1 0x{:04x}", opcode),
                _ => todo!("opcode?: 0x{:04x}", opcode),
            },
            _ => {
                todo!("opcode?: 0x{:04x}", opcode)
            }
        }
    }

    fn jump_to(&mut self, nnn: u16) {
        self.program_counter = nnn as usize;
    }

    fn call_subroutine_at(&mut self, nnn: u16) {
        if self.stack_pointer >= self.stack.len() {
            panic!("stack overflow");
        }
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = nnn as usize;
    }

    fn skip_if_vx_equals_nn(&mut self, x: u8, nn: u8) {
        if self.registers[x as usize] == nn {
            self.program_counter += 2;
        }
    }

    fn skip_if_vx_not_equals_nn(&mut self, x: u8, nn: u8) {
        if self.registers[x as usize] != nn {
            self.program_counter += 2;
        }
    }

    fn skip_if_vx_equals_vy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.program_counter += 2;
        }
    }

    fn set_vx_to_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn set_vx_to_vx_or_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    fn set_vx_to_vx_and_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] &= self.registers[y as usize];
    }

    fn set_vx_to_vx_xor_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    fn add_vy_to_vx(&mut self, x: u8, y: u8) {
        let (v, flag) = self.registers[x as usize].overflowing_add(self.registers[y as usize]);
        self.registers[x as usize] = v;
        self.registers[0xf] = if flag { 1 } else { 0 };
    }

    fn sub_vy_from_vx(&mut self, x: u8, y: u8) {
        let (v, flag) = self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
        self.registers[x as usize] = v;
        self.registers[0xf] = if flag { 0 } else { 1 };
    }
}
fn main() {
    let mut chip = Chip8::new();
    chip.run();
}
