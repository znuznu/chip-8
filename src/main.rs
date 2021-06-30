mod screen;

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

struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    fn new() -> Self {
        Self { keys: [false; 16] }
    }
}

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

        let (x, y, n) = (n2, n3, n4);

        match (n1, n2, n3, n4) {
            (0x00, 0x00, 0x0e, 0x00) => self.execute_cls(),
            (0x00, 0x00, 0x0e, 0x0e) => self.execute_ret(),
            (0x01, _, _, _) => self.execute_jp_nnn(nnn),
            (0x02, _, _, _) => self.execute_call_nnn(nnn),
            (0x03, _, _, _) => self.execute_se_vx_byte(x, kk),
            (0x04, _, _, _) => self.execute_sen_vx_byte(x, kk),
            (0x05, _, _, 0x00) => self.execute_se_vx_vy(x, y),
            (0x05, _, _, _) => self.execute_ld_vx_byte(x, kk),
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

    fn execute_se_vx_byte(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] == kk {
            self.pc += 2;
        }
    }

    fn execute_sen_vx_byte(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] != kk {
            self.pc += 2;
        }
    }

    fn execute_se_vx_vy(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        if vx == vy {
            self.pc += 2;
        }
    }

    fn execute_ld_vx_byte(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = kk;
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
    fn test_se_vx_byte() {
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
    fn test_sen_vx_byte() {
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
    fn test_ld_vx_byte() {
        let mut interpreter = Interpreter::new();
        interpreter.fetch();
        interpreter.decode(0x51AA);

        assert_eq!(interpreter.v[1], 0xAA);
    }
}
