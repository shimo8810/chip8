#[derive(Debug)]
struct Chip8 {
    /// general purpose v (8-bit, x16)
    v: [u8; 16],
    /// memory address register (16-bit)
    i: u16,
    /// program counter (16-bit)
    pc: u16,
    /// stack pointer (8-bit)
    sp: u8,
    /// stack (16-bit, x16)
    stack: [u16; 16],
    /// memory
    mem: [u8; 4096],
}

#[allow(unused_variables)]
impl Chip8 {
    fn new() -> Self {
        Chip8 {
            v: [0u8; 16],
            i: 0,
            pc: 0,
            mem: [0u8; 4096],
            stack: [0u16; 16],
            sp: 0,
        }
    }

    fn run(&mut self) {
        // fetch
        let op1 = self.mem[self.pc as usize] as u16;
        let op2 = self.mem[self.pc as usize + 1] as u16;
        let opcode = op1 << 8 | op2;

        // operand
        let addr = opcode & 0x0FFF;
        let byte = (opcode & 0x00FF) as u8;
        let nibble = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        // decode and execute
        match (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        ) {
            (0x0, 0x0, 0x0, 0x0) => todo!("return?"),
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jp_addr(addr),
            (0x2, _, _, _) => self.call(addr),
            (0x3, _, _, _) => self.se_vx_byte(x, byte),
            (0x4, _, _, _) => self.sne_vx_byte(x, byte),
            (0x5, _, _, 0x0) => self.se_vx_vy(x, y),
            (0x6, _, _, _) => self.ld_vx_byte(x, byte),
            (0x7, _, _, _) => self.add_vx_byte(x, byte),
            (0x8, _, _, 0x0) => self.ld_vx_vy(x, y),
            (0x8, _, _, 0x1) => self.or(x, y),
            (0x8, _, _, 0x2) => self.and(x, y),
            (0x8, _, _, 0x3) => self.xor(x, y),
            (0x8, _, _, 0x4) => self.add_vx_vy(x, y),
            (0x8, _, _, 0x5) => self.sub(x, y),
            (0x8, _, _, 0x6) => self.shr(x),
            (0x8, _, _, 0x7) => self.subn(x, y),
            (0x8, _, _, 0xE) => self.shl(x),
            (0x9, _, _, 0x0) => self.sne_vx_vy(x, y),
            (0xA, _, _, _) => self.ld_i_addr(addr),
            (0xB, _, _, _) => self.jp_v0_addr(addr),
            (0xC, _, _, _) => self.rnd(x, byte),
            (0xD, _, _, _) => self.drw(x, y, nibble),
            (0xE, _, 0x9, 0xE) => self.skp(x),
            (0xE, _, 0xA, 0x1) => self.skpn(x),
            (0xF, _, 0xA, 0x7) => self.ld_vx_dt(x),
            (0xF, _, 0x0, 0xA) => self.ld_vx_k(x),
            (0xF, _, 0x1, 0x5) => self.ld_dt_vx(x),
            (0xF, _, 0x1, 0x8) => self.ld_st_vx(x),
            (0xF, _, 0x1, 0xE) => self.add_i_vx(x),
            (0xF, _, 0x2, 0x9) => self.ld_f_vx(x),
            (0xF, _, 0x3, 0x3) => self.ld_b_vx(x),
            (0xF, _, 0x5, 0x5) => self.ld_i_vx(x),
            (0xF, _, 0x6, 0x5) => self.ld_vx_i(x),
            _ => panic!("miss match opcode"),
        }
    }

    /// clear the display.
    fn cls(&mut self) {
        todo!("opcode 00E0");
    }

    /// return from a subroutine.
    fn ret(&mut self) {
        todo!("opcode 00EE");
    }

    /// jump to location nnn.
    /// opcode: 1nnn
    fn jp_addr(&mut self, addr: u16) {
        self.pc = addr;
    }

    /// call subroutine at nnn
    /// opcode: 2nnn
    fn call(&mut self, addr: u16) {
        if self.sp as usize >= self.stack.len() {
            panic!("stack overflow");
        }
        self.stack[self.sp as usize] = self.pc as u16;
        self.sp += 1;
        self.pc = addr;
    }

    /// skip next instruction if vx == kk
    /// opcode: 3xkk
    fn se_vx_byte(&mut self, x: u8, byte: u8) {
        if self.v[x as usize] == byte {
            self.pc += 2;
        }
    }

    /// skip next instruction if vx != kk
    /// 4xkk
    fn sne_vx_byte(&mut self, x: u8, byte: u8) {
        if self.v[x as usize] != byte {
            self.pc += 2;
        }
    }

    /// skip next instruction if vx == vy
    /// opcode: 5xy0
    fn se_vx_vy(&mut self, x: u8, y: u8) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 2;
        }
    }

    // set vx = kk
    fn ld_vx_byte(&mut self, x: u8, byte: u8) {
        todo!("opcode 6xkk");
    }

    /// set Vx = Vx + kk
    fn add_vx_byte(&mut self, x: u8, byte: u8) {
        todo!("opcode 7xkk");
    }

    /// set vx = vy
    /// opcode: 8xy0
    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
    }

    /// set vx = vx or vy
    /// opcode: 8xy1
    fn or(&mut self, x: u8, y: u8) {
        self.v[x as usize] |= self.v[y as usize];
    }

    /// set vx = vx and vy
    /// opcode: 8xy2
    fn and(&mut self, x: u8, y: u8) {
        self.v[x as usize] &= self.v[y as usize];
    }

    /// set vx = vx xor vy
    /// opcode: 8xy3
    fn xor(&mut self, x: u8, y: u8) {
        self.v[x as usize] ^= self.v[y as usize];
    }

    /// set vx = vx + vy, set vf = carry
    /// opcode: 8xy4
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let (v, flag) = self.v[x as usize].overflowing_add(self.v[y as usize]);
        self.v[x as usize] = v;
        self.v[0xf] = if flag { 1 } else { 0 };
    }

    /// set vx = vx - vy, set vf = not borrow
    /// opcode: 8xy5
    fn sub(&mut self, x: u8, y: u8) {
        let (v, flag) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
        self.v[x as usize] = v;
        // MEMO: 等しいときのフラグに誤り?
        self.v[0xf] = if flag { 0 } else { 1 };
    }

    /// set vx = vx shr 1
    fn shr(&mut self, x: u8) {
        todo!("opcode 8xy6");
    }

    /// set vx = vy - vx, set vf = not borrow
    fn subn(&mut self, x: u8, y: u8) {
        todo!("opcode 8xy7");
    }

    /// set vx = vx shl 1
    fn shl(&mut self, x: u8) {
        todo!("opcode 8xyE");
    }

    /// skip next instrution if vx != vy
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        todo!("opcode 9xy0");
    }

    /// set I = nnn
    fn ld_i_addr(&mut self, addr: u16) {
        todo!("opcode Annn");
    }

    /// jump to location nnn + v0
    fn jp_v0_addr(&mut self, addr: u16) {
        todo!("opecode Bnnn");
    }

    /// set vx = random byte and kk
    fn rnd(&mut self, x: u8, byte: u8) {
        todo!("opcode Cxkk");
    }

    /// display n-byte sprite starting at memory location I at (vx, vy)
    /// set vf = collision.
    fn drw(&mut self, x: u8, y: u8, nibble: u8) {
        todo!("opcode Dxyn");
    }

    /// skip next instruction if key with the value of vx is pressed.
    fn skp(&mut self, x: u8) {
        todo!("opcode Ex9E");
    }

    /// skip next instruction if keyt with the value of vx is not pressed.
    fn skpn(&mut self, x: u8) {
        todo!("opcode ExA1")
    }

    /// set vx = delay timer value
    fn ld_vx_dt(&mut self, x: u8) {
        todo!("opcode Fx07");
    }

    /// wait for a key press, sotre the value of the key in vx
    fn ld_vx_k(&mut self, x: u8) {
        todo!("opcode Fx0A");
    }

    /// set delay timer = vx
    fn ld_dt_vx(&mut self, x: u8) {
        todo!("opcode Fx15");
    }

    /// set soud timer = vx
    fn ld_st_vx(&mut self, x: u8) {
        todo!("opcode Fx18");
    }

    /// set I = I + vx
    fn add_i_vx(&mut self, x: u8) {
        todo!("opcode Fx1E");
    }

    /// set I = location of sprite for digit vx
    fn ld_f_vx(&mut self, x: u8) {
        todo!("opcode Fx29");
    }

    /// store bcd representation of vx in memory locations I, I+1, and I+2
    fn ld_b_vx(&mut self, x: u8) {
        todo!("opcode Fx33");
    }

    /// store v vx throgh vx in memory starting at location I.
    fn ld_i_vx(&mut self, x: u8) {
        todo!("opcode Fx55");
    }

    /// read v v0 through vx from memory starting at location I.
    fn ld_vx_i(&mut self, x: u8) {
        todo!("opcode Fx65");
    }
}
fn main() {
    let mut chip = Chip8::new();
    chip.run();
}
