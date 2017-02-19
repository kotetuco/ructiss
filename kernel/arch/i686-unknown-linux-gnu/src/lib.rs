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

pub fn init_os() {
    let vram_address: u32;
    let screen_x: u16;
    let screen_y: u16;
    unsafe {
        vram_address = *(0x00000ff8 as *mut u32);
        screen_x = *(0x00000ff4 as *const u16);
        screen_y = *(0x00000ff6 as *const u16);
    }

    init_palette();

    let max_offset: u32 = (screen_x * screen_y) as u32;
     for offset in 0..max_offset {
//        write_mem8(vram_address + offset, 0x00);
         let vram: *mut u8 = (vram_address + offset) as *mut u8;
         unsafe {
             // light green
             *vram = 0x02;
         }
     }
}

pub fn sleep_cpu() {
    hlt();
}

//
// private
//

trait SetRGBToIO {
    fn set_rgb_to_io(&self, palette_no:i32);
}

impl SetRGBToIO for RGB {
    fn set_rgb_to_io(&self, palette_no:i32) {
        unsafe {
            io_out8(0x03c8, palette_no);
            io_out8(0x03c9, self.r >> 2);
            io_out8(0x03c9, self.g >> 2);
            io_out8(0x03c9, self.b >> 2);
        }
    //    io_out_u8(0x03c8, palette_no);
    //    io_out_u8(0x03c9, self.r >> 2);
    //    io_out_u8(0x03c9, self.g >> 2);
    //    io_out_u8(0x03c9, self.b >> 2);
    }
}

fn init_palette() {
    let eflags = load_eflags();
    cli();
    RGBDef.black().set_rgb_to_io(0);
    RGBDef.light_red().set_rgb_to_io(1);
    RGBDef.light_green().set_rgb_to_io(2);
    RGBDef.light_yellow().set_rgb_to_io(3);
    RGBDef.light_blue().set_rgb_to_io(4);
    RGBDef.light_purple().set_rgb_to_io(5);
    RGBDef.light_pale_blue().set_rgb_to_io(6);
    RGBDef.white().set_rgb_to_io(7);
    RGBDef.light_gray().set_rgb_to_io(8);
    RGBDef.dark_red().set_rgb_to_io(9);
    RGBDef.dark_green().set_rgb_to_io(10);
    RGBDef.dark_yellow().set_rgb_to_io(11);
    RGBDef.dark_blue().set_rgb_to_io(12);
    RGBDef.dark_purple().set_rgb_to_io(13);
    RGBDef.dark_pale_blue().set_rgb_to_io(14);
    RGBDef.dark_gray().set_rgb_to_io(15);
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
