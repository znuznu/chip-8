pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn is_pressed(&self, index: usize) -> bool {
        return self.keys[index];
    }

    pub fn set_up(&mut self, index: usize) {
        self.keys[index] = false;
    }

    pub fn set_down(&mut self, index: usize) {
        self.keys[index] = true;
    }
}
