pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub const fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn is_pressed(&self, index: usize) -> bool {
        self.keys[index]
    }

    pub fn set_up(&mut self, index: usize) {
        self.keys[index] = false;
    }

    pub fn set_down(&mut self, index: usize) {
        self.keys[index] = true;
    }

    pub fn get_key_pressed(&mut self) -> Option<u8> {
        for (i, key) in self.keys.iter().enumerate() {
            if *key {
                return Some(i as u8);
            }
        }

        None
    }
}
