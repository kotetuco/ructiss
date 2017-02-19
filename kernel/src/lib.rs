//
// kotetuco, 2016
//

#![feature(lang_items)]
#![no_std]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;
extern crate rlibc;
// extern crate arch;
// extern crate arch_arm_none_eabi;
extern crate ructiss_core;

// use arch::sleep_cpu;
// use arch::init_palette;

// #[no_mangle]
// pub extern fn init_os() {
//     init_i686();
// }

extern "C" {
    // #[cfg(any(target_arch = "arm-none-eabi"))]
    pub fn hlt();
}

#[no_mangle]
pub extern "C" fn init_gba() {
    let ioram_address: u32 = 0x04000000;
    let vram_address: u32 = 0x06000000;
    let screen_x: u16 = 240;
    let screen_y: u16 = 160;
    unsafe {
        let video_mode: *mut u8 = ioram_address as *mut u8;
        *video_mode = 0x03; // mode 3
        let bg: *mut u8 = (ioram_address + 1) as *mut u8;
        *bg = 0x04; // BG2
    }

    // draw vram
    let max_offset: u32 = (screen_x * screen_y) as u32;
    for offset in 0..max_offset {
        // 1dot is 2byte.
        let vram: *mut u16 = (vram_address + (offset * 2)) as *mut u16;
        unsafe {
            // green
            *vram = 0x03e0;
        }
    }
    loop {
        unsafe {
            hlt();
        }
    }
}

// fn init_i686() {
//     // write white color to video memory.
//     let vram_address: u32;
//     let screen_x: u16;
//     let screen_y: u16;
//     unsafe {
//         vram_address = *(0x00000ff8 as *mut u32);
//         screen_x = *(0x00000ff4 as *const u16);
//         screen_y = *(0x00000ff6 as *const u16);
//     }
//
//     init_palette();
//
//    let max_offset: u32 = (screen_x * screen_y) as u32;
//     for offset in 0..max_offset {
// //        write_mem8(vram_address + offset, 0x00);
//         let vram: *mut u8 = (vram_address + offset) as *mut u8;
//         unsafe {
//             // light green
//             *vram = 0x02;
//         }
//     }
//
//     loop {
//         sleep_cpu();
//     }
// }

// https://doc.rust-lang.org/book/no-stdlib.html
// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[no_mangle]
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

// #[no_mangle]
// #[lang = "panic_fmt"]
// pub extern fn panic_fmt() -> ! { loop {} }

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() -> ! { loop {} }

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
