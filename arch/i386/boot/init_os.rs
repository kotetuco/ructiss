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
fn debug_print_to_ax(value:u16) {
    unsafe {
        // デバッグ出力
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

    let max_offset: u32 = (screen_x * screen_y) as u32;
    for offset in 0x00000000..max_offset {
        let vram: *mut u8 = (vram_address + offset) as *mut u8;
        unsafe {   
            *vram = 0x0f;
        }
    }
    
    loop {
        hlt()
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
