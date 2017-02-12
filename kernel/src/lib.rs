//
// kotetuco, 2016
//

#![feature(lang_items)]
#![no_std]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;
extern crate rlibc;
extern crate arch;
extern crate ructiss_core;

use arch::sleep_cpu;
use arch::init_palette;

#[no_mangle]
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
            // light green
            *vram = 0x02;
        }
    }

    loop {
        sleep_cpu();
    }
}

// https://doc.rust-lang.org/book/no-stdlib.html
// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[no_mangle]
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt() -> ! { loop {} }

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() -> ! {
    loop {}
}
