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
    drawer.draw_box(20, 20, 100, 100, &Palette::light_red());
    drawer.draw_box(70, 50, 100, 100, &Palette::light_green());
    drawer.draw_box(120, 80, 100, 100, &Palette::light_blue());
    loop {
        sleep_cpu();
    }
}
