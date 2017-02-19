//
// kotetuco, 2016
//

#![feature(lang_items)]
#![no_std]
#![feature(asm)]

extern crate ructiss_core;

use ructiss_core::RGB;
use ructiss_core::RGBDef;

//
// for catching event
//

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt() -> ! { loop {} }

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
    #[cfg(any(target_arch = "x86"))]
    pub fn io_out8(port: i32, data: i32);
}

//
// public Interface
//

pub fn init_os() {
    init_palette();
}

pub trait ScreenDrawer {
    fn new() -> Self;
    fn draw_dot(&self, x:u16, y:u16, color:Palette);
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
            vram_address: unsafe{ *(0x00000ff8 as *mut u32) },
            screen_x: unsafe{ *(0x00000ff4 as *const u16) },
            screen_y: unsafe{ *(0x00000ff6 as *const u16) },
        }
    }

    fn draw_dot(&self, x:u16, y:u16, color:Palette) {
        let offset: u32 = ((y * self.screen_x) + x) as u32;
        let vram: *mut u8 = (self.vram_address + offset) as *mut u8;
        unsafe {
            // light green
            *vram = color.palette_no;
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
    hlt();
}

//
// private
//

pub struct Palette {
    palette_no: u8,
}

impl Palette {
    fn set_palette_no_to_io(&self) {
        unsafe {
            io_out8(0x03c8, self.palette_no as i32);
        }
        // io_out_u8(0x03c8, palette_no);
    }
}

impl RGBDef for Palette {
    fn black() -> Palette { return Palette{ palette_no: 0 } }
    fn light_red() -> Palette { return Palette{ palette_no: 1 } }
    fn light_green() -> Palette { return Palette{ palette_no: 2 } }
    fn light_yellow() -> Palette { return Palette{ palette_no: 3 } }
    fn light_blue() -> Palette { return Palette{ palette_no: 4 } }
    fn light_purple() -> Palette { return Palette{ palette_no: 5 } }
    fn light_pale_blue() -> Palette { return Palette{ palette_no: 6 } }
    fn white() -> Palette { return Palette{ palette_no: 7 } }
    fn light_gray() -> Palette { return Palette{ palette_no: 8 } }
    fn dark_red() -> Palette { return Palette{ palette_no: 9 } }
    fn dark_green() -> Palette { return Palette{ palette_no: 10 } }
    fn dark_yellow() -> Palette { return Palette{ palette_no: 11 } }
    fn dark_blue() -> Palette { return Palette{ palette_no: 12 } }
    fn dark_purple() -> Palette { return Palette{ palette_no: 13 } }
    fn dark_pale_blue() -> Palette { return Palette{ palette_no: 14 } }
    fn dark_gray() -> Palette { return Palette{ palette_no: 15 } }
}

trait RGBIO {
    fn set_rgb_to_io(&self);
}

impl RGBIO for RGB {
    fn set_rgb_to_io(&self) {
        unsafe {
            io_out8(0x03c9, (self.r as i32) >> 2);
            io_out8(0x03c9, (self.g as i32) >> 2);
            io_out8(0x03c9, (self.b as i32) >> 2);
        }
        // io_out_u8(0x03c9, self.r >> 2);
        // io_out_u8(0x03c9, self.g >> 2);
        // io_out_u8(0x03c9, self.b >> 2);
    }
}

fn init_palette() {
    let eflags = load_eflags();
    cli();

    Palette::black().set_palette_no_to_io();
    RGB::black().set_rgb_to_io();

    Palette::light_red().set_palette_no_to_io();
    RGB::light_red().set_rgb_to_io();

    Palette::light_green().set_palette_no_to_io();
    RGB::light_green().set_rgb_to_io();

    Palette::light_yellow().set_palette_no_to_io();
    RGB::light_yellow().set_rgb_to_io();

    Palette::light_blue().set_palette_no_to_io();
    RGB::light_blue().set_rgb_to_io();

    Palette::light_purple().set_palette_no_to_io();
    RGB::light_purple().set_rgb_to_io();

    Palette::light_pale_blue().set_palette_no_to_io();
    RGB::light_pale_blue().set_rgb_to_io();

    Palette::white().set_palette_no_to_io();
    RGB::white().set_rgb_to_io();

    Palette::light_gray().set_palette_no_to_io();
    RGB::light_gray().set_rgb_to_io();

    Palette::dark_red().set_palette_no_to_io();
    RGB::dark_red().set_rgb_to_io();

    Palette::dark_green().set_palette_no_to_io();
    RGB::dark_green().set_rgb_to_io();

    Palette::dark_yellow().set_palette_no_to_io();
    RGB::dark_yellow().set_rgb_to_io();

    Palette::dark_blue().set_palette_no_to_io();
    RGB::dark_blue().set_rgb_to_io();

    Palette::dark_purple().set_palette_no_to_io();
    RGB::dark_purple().set_rgb_to_io();

    Palette::dark_pale_blue().set_palette_no_to_io();
    RGB::dark_pale_blue().set_rgb_to_io();

    Palette::dark_gray().set_palette_no_to_io();
    RGB::dark_gray().set_rgb_to_io();

    store_eflags(eflags);
}

#[cfg(any(target_arch = "x86"))]
fn hlt() {
    unsafe {
        asm!("hlt" :::: "intel");
    }
}

#[cfg(any(target_arch = "x86"))]
fn cli() {
    unsafe {
        asm!("cli" :::: "intel");
    }
}

#[cfg(any(target_arch = "x86"))]
fn load_eflags() -> u32 {
    let eflags: u32;
    unsafe {
        asm!("pushfd
              pop $0"
             : "=r"(eflags)
             :
             :
             : "intel");
    }
    return eflags;
}

#[cfg(any(target_arch = "x86"))]
fn store_eflags(eflags: u32) {
    unsafe {
        asm!("push $0
              popfd"
             :
             : "r"(eflags)
             :
             : "intel");
    }
}

#[allow(dead_code)]
#[cfg(any(target_arch = "x86"))]
fn debug_print_to_eax(value:u32) {
    unsafe {
        asm!("mov eax, $0"
             :
             : "r"(value)
             :
             : "intel");
    }
    loop {
        hlt()
    }
}

#[allow(dead_code)]
#[cfg(any(target_arch = "x86"))]
fn debug_print_to_ax(value:u16) {
    unsafe {
        asm!("mov ax, $0"
             :
             : "r"(value)
             :
             : "intel");
    }
    loop {
        hlt()
    }
}

// #[cfg(any(target_arch = "x86"))]
// fn write_mem8(addr:u32, data:u8) {
//     unsafe {
//         asm!("mov BYTE PTR [$0], $1"
//              :
//              : "r"(addr), "r"(data)
//              :
//              : "intel");
//     }
// }
//
// #[cfg(any(target_arch = "x86"))]
// fn io_out_u8(port:i32, data:i32) {
//     unsafe {
//         asm!("mov dx, [esp + 4]
//               mov al, [esp + 8]
//               out dx, al"
//              :
//              : "r"(port), "r"(data)
//              : "eax", "edx"
//              : "intel");
//     }
// }
//
// //#[no_mangle]
// #[cfg(any(target_arch = "x86"))]
// fn io_out_i32(port:i32, data:i32) {
//     unsafe {
//         asm!("mov edx, $0
//               mov eax, $1
//               out dx, eax"
//              :
//              : "r"(port), "r"(data)
//              : "eax", "edx"
//              : "intel");
//     }
// }
