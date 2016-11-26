//
// kotetuco, 2016
// 

#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![feature(no_std)]
#![no_std]
#![feature(asm)]

use core::mem;

#[no_mangle]
#[cfg(any(target_arch = "x86"))]
fn hlt() {
    unsafe {
        asm!("hlt" :::: "intel");
    }
}

#[no_mangle]
#[cfg(any(target_arch = "x86"))]
fn cli() {
    unsafe {
        asm!("cli" :::: "intel");
    }
}

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
#[cfg(any(target_arch = "x86"))]
fn write_mem8(addr:u32, data:u8) {
    unsafe {
        asm!("mov BYTE PTR [$0], $1"
             :
             : "r"(addr), "r"(data)
             :
             : "intel");
    }
}

#[no_mangle]
#[cfg(any(target_arch = "x86"))]
fn io_out_u8(port:u32, data:u8) {
    unsafe {
        asm!("mov edx, $0
              mov al, $1
              out dx, ax"
             :
             : "r"(port), "r"(data)
             : "eax", "edx"
             : "intel");
    }
}

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
fn init_palette() {
    let eflags = load_eflags();
    cli();
    io_out_u8(0x03c8, 0);
    RGBDef.black().setRGBToIO();
    RGBDef.lightRed().setRGBToIO();
    RGBDef.lightYellow().setRGBToIO();
    RGBDef.lightBlue().setRGBToIO();
    RGBDef.lightPurple().setRGBToIO();
    RGBDef.lightPaleBlue().setRGBToIO();
    RGBDef.white().setRGBToIO();
    RGBDef.lightGray().setRGBToIO();
    RGBDef.darkRed().setRGBToIO();
    RGBDef.darkGreen().setRGBToIO();
    RGBDef.darkYellow().setRGBToIO();
    RGBDef.darkBlue().setRGBToIO();
    RGBDef.darkPaleBlue().setRGBToIO();
    RGBDef.darkPaleBlue().setRGBToIO();
    RGBDef.darkGray().setRGBToIO();
    store_eflags(eflags);
}

#[no_mangle]
#[start]
pub extern fn init_os() {
    // write white color to video memory.
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
            *vram = 0x02;
        }
    }
    
    loop {
        hlt()
    }
}

struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

struct RGBDef;

impl RGBDef {
    fn black(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x00,};
    }

    fn lightRed(&self) -> RGB {
        return RGB { r: 0xff, g: 0x00, b: 0x00,};
    }

    fn lightGreen(&self) -> RGB {
        return RGB { r: 0x00, g: 0xff, b: 0x00,};
    }

    fn lightYellow(&self) -> RGB {
        return RGB { r: 0xff, g: 0xff, b: 0x00,};
    }

    fn lightBlue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0xff,};
    }

    fn lightPurple(&self) -> RGB {
        return RGB { r: 0xff, g: 0x00, b: 0xff,};
    }

    fn lightPaleBlue(&self) -> RGB {
        return RGB { r: 0x00, g: 0xff, b: 0xff,};
    }

    fn white(&self) -> RGB {
        return RGB { r: 0xff, g: 0xff, b: 0xff,};
    }

    fn lightGray(&self) -> RGB {
        return RGB { r: 0xc6, g: 0xc6, b: 0xc6,};
    }

    fn darkRed(&self) -> RGB {
        return RGB { r: 0x84, g: 0x00, b: 0x00,};
    }

    fn darkGreen(&self) -> RGB {
        return RGB { r: 0x00, g: 0x84, b: 0x00,};
    }

    fn darkYellow(&self) -> RGB {
        return RGB { r: 0x84, g: 0x84, b: 0x00,};
    }

    fn darkBlue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x84,};
    }

    fn darkPurple(&self) -> RGB {
        return RGB { r: 0x84, g: 0x00, b: 0x84,};
    }

    fn darkPaleBlue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x84, b: 0x84,};
    }

    fn darkGray(&self) -> RGB {
        return RGB { r: 0x84, g: 0x84, b: 0x84,};
    }
}

trait SetRGBToIO {
    fn setRGBToIO(&self);
}

impl SetRGBToIO for RGB {
    fn setRGBToIO(&self) {
        io_out_u8(0x03c9, self.r << 2);//R
        io_out_u8(0x03c9, self.g << 2);//G
        io_out_u8(0x03c9, self.b << 2);//B
    }
}

// https://doc.rust-lang.org/book/no-stdlib.html
// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }
