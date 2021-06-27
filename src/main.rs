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
}

// TODO Read instructions - Decode - Execute

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
}
