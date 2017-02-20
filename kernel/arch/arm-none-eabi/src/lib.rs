//
// kotetuco, 2016
//

#![feature(lang_items)]
#![no_std]

extern crate ructiss_core;

use ructiss_core::RGB;

//
// for catching event
//

#[no_mangle]
#[lang = "panic_fmt"]
pub extern "C" fn rust_begin_panic(_msg: ::core::fmt::Arguments,
                                   _file: &'static str,
                                   _line: u32)
                                   -> ! {
    loop {}
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() {
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() {
}

#[no_mangle]
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_Resume() -> ! { loop {} }

//
// import asm functions
//

extern "C" {
    #[cfg(any(target_arch = "arm"))]
    pub fn hlt();
}

//
// public Interface
//

pub fn init_os() {
    init_graphic();
}

pub trait ScreenDrawer {
    fn new() -> Self;
    fn draw_dot(&self, x:u16, y:u16, color:&Palette);
    fn draw_box(&self, x:u16, y:u16, width:u16, height:u16, color:&Palette);
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

pub struct ArchScreenDrawer {
    vram_address: u32,
    screen_x: u16,
    screen_y: u16,
}

impl ScreenDrawer for ArchScreenDrawer {
    fn new() -> Self {
        ArchScreenDrawer {
            vram_address: 0x06000000,
            screen_x: 240,
            screen_y: 160,
        }
    }

    fn draw_dot(&self, x:u16, y:u16, color:&Palette) {
        let offset: u32 = ((y * self.screen_x) + x) as u32;
        let vram: *mut u16 = (self.vram_address + (offset * 2)) as *mut u16;
        unsafe {
            // light green
            *vram = color.convert_16bit_rgb();
        }
    }

    fn draw_box(&self, x:u16, y:u16, width:u16, height:u16, color:&Palette) {
        for offset_y in 0..height {
            for offset_x in 0..width {
                let valid_x = if x + offset_x > self.screen_x { self.screen_x } else { x + offset_x };
                let valid_y = if y + offset_y > self.screen_y { self.screen_y } else { y + offset_y };
                self.draw_dot(valid_x, valid_y, color);
            }
        }
    }

    fn width(&self) -> u16 {
        return self.screen_x;
    }

    fn height(&self) -> u16 {
        return self.screen_y;
    }
}

pub fn sleep_cpu() {
    unsafe {
        // hlt();
    }
}

//
// private
//

pub type Palette = RGB;

trait GBARGB {
    fn convert_16bit_rgb(&self) -> u16;
}

impl GBARGB for RGB {
    fn convert_16bit_rgb(&self) -> u16{
        return (((self.b >> 3) as u16) << 10) + (((self.g >> 3) as u16) << 5) + (self.r >> 3) as u16;
    }
}

fn init_graphic() {
    let ioram_address: u32 = 0x04000000;
    unsafe {
        let video_mode: *mut u8 = ioram_address as *mut u8;
        *video_mode = 0x03; // mode 3
        let bg: *mut u8 = (ioram_address + 1) as *mut u8;
        *bg = 0x04; // BG2
    }
}
