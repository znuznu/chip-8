mod keypad;
mod screen;

use crate::keypad::Keypad;
use crate::screen::Screen;

const FONTS_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// TODO maybe structs for:
// - memory
// - stack -> could use a wrapper for a vec with Result

struct Interpreter {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    dtimer: u8,
    stimer: u8,
    screen: Screen,
    keypad: Keypad,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            dtimer: 0,
            stimer: 0,
            screen: Screen::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn init(&mut self) {
        // Load fonts in memory
        for i in 0..80 {
            self.memory[i] = FONTS_SPRITES[i];
        }

        self.v = [0; 16];
        self.i = 0;
        // The CHIP-8 program space goes from 0x200 to 0xFFF
        self.pc = 0x200;
        self.stack = [0; 16];
        self.sp = 0;
        self.dtimer = 0;
        self.stimer = 0;
        // TODO probably better to avoid a new() here
        self.screen = Screen::new();
    }

    pub fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;
        let instruction: u16 = (self.memory[pc] as u16) << 8 | (self.memory[(pc + 1)] as u16);
        self.pc += 2;

        return instruction;
    }

    pub fn decode(&mut self, instruction: u16) {
        let n1 = ((instruction & 0xF000) >> 12) as u8;
        let n2 = ((instruction & 0x0F00) >> 8) as u8;
        let n3 = ((instruction & 0x00F0) >> 4) as u8;
        let n4 = (instruction & 0x000F) as u8;

        let nnn = instruction & 0x0FFF;
        let kk = (instruction & 0x00FF) as u8;

        let (x, y, n) = (n2 as usize, n3 as usize, n4);

        match (n1, n2, n3, n4) {
            (0x00, 0x00, 0x0e, 0x00) => self.execute_cls(),
            (0x00, 0x00, 0x0e, 0x0e) => self.execute_ret(),
            (0x01, _, _, _) => self.execute_jp_nnn(nnn),
            (0x02, _, _, _) => self.execute_call_nnn(nnn),
            (0x03, _, _, _) => self.execute_se_vx_kk(x, kk),
            (0x04, _, _, _) => self.execute_sne_vx_kk(x, kk),
            (0x05, _, _, 0x00) => self.execute_se_vx_vy(x, y),
            (0x06, _, _, _) => self.execute_ld_vx_kk(x, kk),
            (0x07, _, _, _) => self.execute_add_vx_kk(x, kk),
            (0x08, _, _, 0x00) => self.execute_ld_vx_vy(x, y),
            (0x08, _, _, 0x01) => self.execute_or_vx_vy(x, y),
            (0x08, _, _, 0x02) => self.execute_and_vx_vy(x, y),
            (0x08, _, _, 0x03) => self.execute_xor_vx_vy(x, y),
            (0x08, _, _, 0x04) => self.execute_add_vx_vy(x, y),
            (0x08, _, _, 0x05) => self.execute_sub_vx_vy(x, y),
            (0x08, _, _, 0x06) => self.execute_shr_vx_vy(x),
            (0x08, _, _, 0x07) => self.execute_subn_vx_vy(x, y),
            (0x08, _, _, 0x0E) => self.execute_shl_vx_vy(x),
            (0x09, _, _, 0x00) => self.execute_sne_vx_vy(x, y),
            (0x0A, _, _, _) => self.execute_ld_i_nnn(nnn),
            (0x0B, _, _, _) => self.execute_jp_v0_nnn(nnn),
            (0x0C, _, _, _) => self.execute_rnd_vx_kk(x, kk),
            (0x0D, _, _, _) => self.execute_drw_vx_vy_n(x, y, n),
            (0x0E, _, 0x09, 0x0E) => self.execute_skp_vx(x),
            (0x0E, _, 0x0A, 0x01) => self.execute_skpn_vx(x),
            (0x0F, _, 0x00, 0x07) => self.execute_ld_vx_dt(x),
            (0x0F, _, 0x00, 0x0A) => self.execute_ld_vx_k(x),
            (0x0F, _, 0x01, 0x05) => self.execute_ld_dt_vx(x),
            (0x0F, _, 0x01, 0x08) => self.execute_ld_st_vx(x),
            (0x0F, _, 0x01, 0x0E) => self.execute_add_i_vx(x),
            _ => (),
        }
    }

    fn execute_cls(&mut self) {
        self.screen.clear();
    }

    fn execute_ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn execute_jp_nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn execute_call_nnn(&mut self, nnn: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn execute_se_vx_kk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 2;
        }
    }

    fn execute_sne_vx_kk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 2;
        }
    }

    fn execute_sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    fn execute_se_vx_vy(&mut self, x: usize, y: usize) {
        let vx = self.v[x];
        let vy = self.v[y];

        if vx == vy {
            self.pc += 2;
        }
    }

    fn execute_ld_vx_kk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    fn execute_add_vx_kk(&mut self, x: usize, kk: u8) {
        self.v[x] += kk;
    }

    fn execute_add_vx_vy(&mut self, x: usize, y: usize) {
        let (result, overflow) = self.v[y].overflowing_add(self.v[x]);
        self.v[15] = if overflow { 1 } else { 0 };
        self.v[x] = result;
    }

    fn execute_ld_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn execute_or_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] | self.v[y];
    }

    fn execute_and_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] & self.v[y];
    }

    fn execute_xor_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] ^ self.v[y];
    }

    fn execute_sub_vx_vy(&mut self, x: usize, y: usize) {
        self.v[15] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }

    fn execute_subn_vx_vy(&mut self, x: usize, y: usize) {
        self.v[15] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    }

    fn execute_shr_vx_vy(&mut self, x: usize) {
        self.v[15] = self.v[x] & 0x1;
        self.v[x] >>= 1;
    }

    fn execute_shl_vx_vy(&mut self, x: usize) {
        self.v[15] = (self.v[x] & 0x80) >> 7;
        self.v[x] <<= 1;
    }

    fn execute_ld_i_nnn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn execute_jp_v0_nnn(&mut self, nnn: u16) {
        self.pc = nnn + self.v[0] as u16;
    }

    fn execute_rnd_vx_kk(&mut self, x: usize, kk: u8) {
        // TODO Use a rng lib
        let rand = 0;
        self.v[x] = rand & kk;
    }

    fn execute_drw_vx_vy_n(&mut self, x: usize, y: usize, n: u8) {
        todo!();
    }

    fn execute_skp_vx(&mut self, x: usize) {
        if self.keypad.is_pressed(self.v[x] as usize) {
            self.pc += 2;
        }
    }

    fn execute_skpn_vx(&mut self, x: usize) {
        if !self.keypad.is_pressed(self.v[x] as usize) {
            self.pc += 2;
        }
    }

    fn execute_ld_vx_dt(&mut self, x: usize) {
        self.v[x] = self.dtimer;
    }

    fn execute_ld_vx_k(&mut self, x: usize) {
        match self.keypad.get_key_pressed() {
            Some(i) => self.v[x] = i,
            None => self.pc -= 2,
        }
    }

    fn execute_ld_dt_vx(&mut self, x: usize) {
        self.dtimer = self.v[x];
    }

    fn execute_ld_st_vx(&mut self, x: usize) {
        self.stimer = self.v[x];
    }

    fn execute_add_i_vx(&mut self, x: usize) {
        self.i += self.v[x] as u16;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Interpreter;

    #[test]
    fn test_ret() {
        let mut interpreter = Interpreter::new();
        interpreter.sp = 12;
        interpreter.stack[11] = 0x4444;
        interpreter.decode(0x00EE);

        assert_eq!(interpreter.pc, 0x4444);
        assert_eq!(interpreter.sp, 11);
    }

    #[test]
    fn test_jp_nnn() {
        let mut interpreter = Interpreter::new();
        interpreter.decode(0x16FF);

        assert_eq!(interpreter.pc, 0x6FF);
    }

    #[test]
    fn test_call_nnn() {
        let mut interpreter = Interpreter::new();
        interpreter.sp = 11;
        interpreter.stack[12] = 0x4444;
        interpreter.pc = 0x66;
        interpreter.decode(0x2AAA);

        assert_eq!(interpreter.pc, 0x0AAA);
        assert_eq!(interpreter.sp, 12);
        assert_eq!(interpreter.stack[11], 0x66);
    }

    #[test]
    fn test_se_vx_kk() {
        let mut interpreter = Interpreter::new();
        interpreter.pc = 2;
        interpreter.v[0] = 0xAA;

        interpreter.fetch();
        interpreter.decode(0x30AA);
        assert_eq!(interpreter.pc, 6);

        interpreter.fetch();
        interpreter.decode(0x30AB);
        assert_eq!(interpreter.pc, 8);
    }

    #[test]
    fn test_sne_vx_kk() {
        let mut interpreter = Interpreter::new();
        interpreter.pc = 2;
        interpreter.v[0] = 0xAA;

        interpreter.fetch();
        interpreter.decode(0x40AA);
        assert_eq!(interpreter.pc, 4);

        interpreter.fetch();
        interpreter.decode(0x40AB);
        assert_eq!(interpreter.pc, 8);
    }

    #[test]
    fn test_sne_vx_vy() {
        let mut interpreter = Interpreter::new();
        interpreter.pc = 2;
        interpreter.v[0] = 0xAA;
        interpreter.v[1] = 0xAA;

        interpreter.decode(0x9010);
        assert_eq!(interpreter.pc, 2);

        interpreter.decode(0x9020);
        assert_eq!(interpreter.pc, 4);
    }

    #[test]
    fn test_se_vx_vy() {
        let mut interpreter = Interpreter::new();
        interpreter.pc = 2;
        interpreter.v[0] = 0xAA;
        interpreter.v[1] = 0xAA;
        interpreter.v[2] = 0x77;

        interpreter.fetch();
        interpreter.decode(0x5010);
        assert_eq!(interpreter.pc, 6);

        interpreter.fetch();
        interpreter.decode(0x5020);
        assert_eq!(interpreter.pc, 8);
    }

    #[test]
    fn test_ld_vx_kk() {
        let mut interpreter = Interpreter::new();
        interpreter.decode(0x61AA);

        assert_eq!(interpreter.v[1], 0xAA);
    }

    #[test]
    fn test_add_vx_kk() {
        let mut interpreter = Interpreter::new();
        interpreter.v[3] = 2;
        interpreter.decode(0x73AA);

        assert_eq!(interpreter.v[3], 0xAA + 2);
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut interpreter = Interpreter::new();
        interpreter.v[3] = 2;
        interpreter.v[5] = 5;
        interpreter.decode(0x8350);

        assert_eq!(interpreter.v[3], 5);
    }

    #[test]
    fn test_or_vx_vy() {
        let mut interpreter = Interpreter::new();
        interpreter.v[1] = 0x0B;
        interpreter.v[2] = 0x03;

        interpreter.decode(0x8121);
        assert_eq!(interpreter.v[1], 11);
    }

    #[test]
    fn test_and_vx_vy() {
        let mut interpreter = Interpreter::new();
        interpreter.v[1] = 0x0B;
        interpreter.v[2] = 0x03;

        interpreter.decode(0x8122);
        assert_eq!(interpreter.v[1], 3);
    }

    #[test]
    fn test_xor_vx_vy() {
        let mut interpreter = Interpreter::new();
        interpreter.v[1] = 0x0B;
        interpreter.v[2] = 0x03;

        interpreter.decode(0x8123);
        assert_eq!(interpreter.v[1], 8);
    }

    #[test]
    fn test_add_vx_vy() {
        let mut interpreter = Interpreter::new();

        interpreter.v[1] = 0xF;
        interpreter.v[2] = 0x3;
        interpreter.decode(0x8124);

        let result_without_overflow = (0xF as u16 + 0x3 as u16) as u8;

        assert_eq!(interpreter.v[1], result_without_overflow);
        assert_eq!(interpreter.v[15], 0);

        interpreter.v[1] = 0xFF;
        interpreter.v[2] = 0x03;
        interpreter.decode(0x8124);

        let result_with_overflow = (0xFF + 0x03) as u8;

        assert_eq!(interpreter.v[1], result_with_overflow);
        assert_eq!(interpreter.v[15], 1);
    }

    #[test]
    fn test_sub_vx_vy() {
        let mut interpreter = Interpreter::new();

        interpreter.v[1] = 0xF;
        interpreter.v[2] = 0x3;
        interpreter.decode(0x8125);

        assert_eq!(interpreter.v[1], 0x0C);
        assert_eq!(interpreter.v[15], 1);

        interpreter.v[1] = 0x14;
        interpreter.v[2] = 0xFF;
        interpreter.decode(0x8125);

        assert_eq!(interpreter.v[1], 0x15);
        assert_eq!(interpreter.v[15], 0);
    }

    #[test]
    fn test_subn_vx_vy() {
        let mut interpreter = Interpreter::new();

        interpreter.v[1] = 0xF;
        interpreter.v[2] = 0x3;
        interpreter.decode(0x8127);

        assert_eq!(interpreter.v[1], 0xF4);
        assert_eq!(interpreter.v[15], 0);

        interpreter.v[1] = 0x0E;
        interpreter.v[2] = 0xFF;
        interpreter.decode(0x8127);

        assert_eq!(interpreter.v[1], 0xF1);
        assert_eq!(interpreter.v[15], 1);
    }

    #[test]
    fn test_shr_vx_vy() {
        let mut interpreter = Interpreter::new();

        interpreter.v[1] = 0x0E;
        interpreter.decode(0x8126);

        assert_eq!(interpreter.v[15], 0);
        assert_eq!(interpreter.v[1], 0x07);

        interpreter.v[1] = 0x0F;
        interpreter.decode(0x8126);

        assert_eq!(interpreter.v[15], 1);
        assert_eq!(interpreter.v[1], 0x07);
    }

    #[test]
    fn test_shl_vx_vy() {
        let mut interpreter = Interpreter::new();

        interpreter.v[1] = 0b01110000;
        interpreter.decode(0x812E);

        assert_eq!(interpreter.v[15], 0);
        assert_eq!(interpreter.v[1], 0b11100000);

        interpreter.v[1] = 0b11000000;
        interpreter.decode(0x812E);

        assert_eq!(interpreter.v[15], 1);
        assert_eq!(interpreter.v[1], 0b10000000);
    }

    #[test]
    fn test_ld_i_nnn() {
        let mut interpreter = Interpreter::new();

        interpreter.decode(0xA123);

        assert_eq!(interpreter.i, 0x123);
    }

    #[test]
    fn test_jp_v0_nnn() {
        let mut interpreter = Interpreter::new();
        interpreter.v[0] = 0x04;

        interpreter.decode(0xB130);

        assert_eq!(interpreter.pc, 0x134);
    }

    #[test]
    fn test_skp_vx() {
        let mut interpreter = Interpreter::new();
        interpreter.pc = 2;
        interpreter.v[1] = 1;
        interpreter.v[2] = 2;
        interpreter.keypad.set_down(1);

        interpreter.decode(0xE19E);
        assert_eq!(interpreter.pc, 4);

        interpreter.decode(0xE29E);
        assert_eq!(interpreter.pc, 4);
    }

    #[test]
    fn test_skpn_vx() {
        let mut interpreter = Interpreter::new();
        interpreter.pc = 2;
        interpreter.v[1] = 1;
        interpreter.v[2] = 2;
        interpreter.keypad.set_down(1);

        interpreter.decode(0xE1A1);
        assert_eq!(interpreter.pc, 2);

        interpreter.decode(0xE2A1);
        assert_eq!(interpreter.pc, 4);
    }

    #[test]
    fn test_ld_vx_dt() {
        let mut interpreter = Interpreter::new();
        interpreter.dtimer = 0x01;
        interpreter.decode(0xF107);

        assert_eq!(interpreter.v[1], 0x01);
    }

    #[test]
    fn test_ld_vx_k() {
        let mut interpreter = Interpreter::new();
        interpreter.fetch();
        interpreter.decode(0xF10A);

        assert_eq!(interpreter.pc, 0);

        interpreter.fetch();
        interpreter.keypad.set_down(1);
        interpreter.decode(0xF10A);

        assert_eq!(interpreter.pc, 2);
        assert_eq!(interpreter.v[1], 1);
    }

    #[test]
    fn test_ld_dt_vx() {
        let mut interpreter = Interpreter::new();
        interpreter.v[1] = 5;
        interpreter.decode(0xF115);

        assert_eq!(interpreter.dtimer, 5);
    }

    #[test]
    fn test_ld_st_vx() {
        let mut interpreter = Interpreter::new();
        interpreter.v[1] = 10;
        interpreter.decode(0xF118);

        assert_eq!(interpreter.stimer, 10);
    }

    #[test]
    fn test_add_i_vx() {
        let mut interpreter = Interpreter::new();
        interpreter.v[1] = 9;
        interpreter.i = 4;

        interpreter.decode(0xF11E);
        assert_eq!(interpreter.i, 13);
    }
}
