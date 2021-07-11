pub struct Screen {
    pub pixels: [u8; 2048],
}

pub enum PixelState {
    On,
    Off,
}

impl Screen {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub const fn new() -> Self {
        Self { pixels: [0; 2048] }
    }

    pub fn clear(&mut self) {
        for x in 0..Screen::WIDTH {
            for y in 0..Screen::HEIGHT {
                self.update_pixel((x, y), PixelState::Off);
            }
        }
    }

    pub fn update_pixel(&mut self, (x, y): (usize, usize), state: PixelState) {
        match state {
            PixelState::On => self.pixels[x + y * Screen::WIDTH] = 1,
            PixelState::Off => self.pixels[x + y * Screen::WIDTH] = 0,
        }
    }

    pub fn get_pixel_state(&self, (x, y): (usize, usize)) -> PixelState {
        match self.pixels[x + y * Screen::WIDTH] {
            0 => PixelState::Off,
            // It would probably be better to use a Result here...
            _ => PixelState::On,
        }
    }
}
