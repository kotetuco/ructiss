//
// kotetuco, 2016
//

#![feature(lang_items)]
#![no_std]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;
extern crate rlibc;
extern crate ructiss_core;

#[allow(unused_imports)]
#[cfg(all(not(test), target_arch = "arm"))]
#[macro_use]
extern crate arch_arm_none_eabi as arch;

#[allow(unused_imports)]
#[cfg(all(not(test), target_arch = "x86"))]
#[macro_use]
extern crate arch_i686_unknown_linux_gnu as arch;

use ructiss_core::RGBDef;
use arch::sleep_cpu;
use arch::init_os;
use arch::ArchScreenDrawer;
use arch::ScreenDrawer;
use arch::Palette;

#[no_mangle]
pub extern "C" fn os_main() {
    init_os();
    let drawer: ArchScreenDrawer = ArchScreenDrawer::new();
    let width: u16 = drawer.width();
    let height: u16 = drawer.height();
    for y in 0..height {
        for x in 0..width {
            drawer.draw_dot(x, y, Palette::white());
        }
    }
    loop {
        sleep_cpu();
    }
}
