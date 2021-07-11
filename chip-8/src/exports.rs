use crate::interpreter::{screen::Screen, Interpreter};

static mut CHIP8: Interpreter = Interpreter::new();

#[no_mangle]
pub unsafe fn init() {
    CHIP8.init();
}

#[no_mangle]
pub unsafe fn get_memory() -> &'static [u8; 4096] {
    &CHIP8.memory
}

#[no_mangle]
pub unsafe fn get_pixels() -> &'static [u8; 2048] {
    &CHIP8.screen.pixels
}

#[no_mangle]
pub unsafe fn tick() {
    CHIP8.tick();
}

#[no_mangle]
pub unsafe fn cycle() {
    &CHIP8.cycle();
}

#[no_mangle]
pub unsafe fn set_key_down(key: u8) {
    CHIP8.keypad.set_down(key.into());
}

#[no_mangle]
pub unsafe fn set_key_up(key: u8) {
    CHIP8.keypad.set_up(key.into());
}

#[no_mangle]
pub unsafe fn get_width() -> usize {
    Screen::WIDTH
}

#[no_mangle]
pub unsafe fn get_height() -> usize {
    Screen::HEIGHT
}
