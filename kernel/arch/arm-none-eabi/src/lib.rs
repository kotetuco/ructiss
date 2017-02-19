//
// kotetuco, 2016
//

#![feature(lang_items)]
#![no_std]

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

pub fn sleep_cpu() {
    unsafe {
        // hlt();
    }
}

//
// private
//

fn init_graphic() {
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
}
