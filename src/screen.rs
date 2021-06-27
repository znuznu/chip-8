pub struct Screen {
    pixels: [u8; 2048],
}

pub enum PixelState {
    On,
    Off,
}

impl Screen {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 32;

    pub fn new() -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::{PixelState, Screen};

    #[test]
    fn test_clear() {
        let mut screen = Screen::new();
        screen.pixels = [1; 2048];

        screen.clear();

        assert_eq!(screen.pixels, [0; 2048]);
    }

    #[test]
    fn test_update_pixel() {
        let mut screen = Screen::new();

        screen.update_pixel((0, 0), PixelState::On);
        screen.update_pixel((0, 10), PixelState::On);
        screen.update_pixel((1, 0), PixelState::On);

        let mut expected_pixels = [0; 2048];
        expected_pixels[0] = 1;
        expected_pixels[10] = 1;
        expected_pixels[64] = 1;

        assert_eq!(screen.pixels, expected_pixels);
    }
}
