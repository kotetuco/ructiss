//
// kotetuco, 2016
// 

#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![no_std]
#![feature(asm)]

extern {
    // Our C function definitions!
    pub fn io_out8(port: i32, data: i32);
}

//#[no_mangle]
#[cfg(any(target_arch = "x86"))]
fn hlt() {
    unsafe {
        asm!("hlt" :::: "intel");
    }
}

//#[no_mangle]
#[cfg(any(target_arch = "x86"))]
fn cli() {
    unsafe {
        asm!("cli" :::: "intel");
    }
}

//#[no_mangle]
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

//#[no_mangle]
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

//#[no_mangle]
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

//#[no_mangle]
#[cfg(any(target_arch = "x86"))]
fn io_out_u8(port:i32, data:i32) {
    unsafe {
        asm!("mov dx, [esp + 4]
              mov al, [esp + 8]
              out dx, al"
             :
             : "r"(port), "r"(data)
             : "eax", "edx"
             : "intel");
    }
}

//#[no_mangle]
#[cfg(any(target_arch = "x86"))]
fn io_out_i32(port:i32, data:i32) {
    unsafe {
        asm!("mov edx, $0
              mov al, $1
              out dx, al"
             :
             : "r"(port), "r"(data)
             : "eax", "edx"
             : "intel");
    }
}

//#[no_mangle]
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

//#[no_mangle]
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

//#[no_mangle]
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
            // light green
            *vram = 0x02;
        }
    }
    
    loop {
        hlt()
    }
}

struct RGB {
    r: i32,
    g: i32,
    b: i32,
}

struct RGBDef;

impl RGBDef {
    fn black(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x00,};
    }

    fn light_red(&self) -> RGB {
        return RGB { r: 0xff, g: 0x00, b: 0x00,};
    }

    fn light_green(&self) -> RGB {
        return RGB { r: 0x00, g: 0xff, b: 0x00,};
    }

    fn light_yellow(&self) -> RGB {
        return RGB { r: 0xff, g: 0xff, b: 0x00,};
    }

    fn light_blue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0xff,};
    }

    fn light_purple(&self) -> RGB {
        return RGB { r: 0xff, g: 0x00, b: 0xff,};
    }

    fn light_pale_blue(&self) -> RGB {
        return RGB { r: 0x00, g: 0xff, b: 0xff,};
    }

    fn white(&self) -> RGB {
        return RGB { r: 0xff, g: 0xff, b: 0xff,};
    }

    fn light_gray(&self) -> RGB {
        return RGB { r: 0xc6, g: 0xc6, b: 0xc6,};
    }

    fn dark_red(&self) -> RGB {
        return RGB { r: 0x84, g: 0x00, b: 0x00,};
    }

    fn dark_green(&self) -> RGB {
        return RGB { r: 0x00, g: 0x84, b: 0x00,};
    }

    fn dark_yellow(&self) -> RGB {
        return RGB { r: 0x84, g: 0x84, b: 0x00,};
    }

    fn dark_blue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x84,};
    }

    fn dark_purple(&self) -> RGB {
        return RGB { r: 0x84, g: 0x00, b: 0x84,};
    }

    fn dark_pale_blue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x84, b: 0x84,};
    }

    fn dark_gray(&self) -> RGB {
        return RGB { r: 0x84, g: 0x84, b: 0x84,};
    }
}

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
//        io_out_u8(0x03c8, palette_no);
//        io_out_u8(0x03c9, self.r >> 2);
//        io_out_u8(0x03c9, self.g >> 2);
//        io_out_u8(0x03c9, self.b >> 2);
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
