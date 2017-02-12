//
// kotetuco, 2016
//

#![feature(lang_items)]
#![no_std]

pub struct RGB {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

pub struct RGBDef;

impl RGBDef {
    pub fn black(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x00,};
    }

    pub fn light_red(&self) -> RGB {
        return RGB { r: 0xff, g: 0x00, b: 0x00,};
    }

    pub fn light_green(&self) -> RGB {
        return RGB { r: 0x00, g: 0xff, b: 0x00,};
    }

    pub fn light_yellow(&self) -> RGB {
        return RGB { r: 0xff, g: 0xff, b: 0x00,};
    }

    pub fn light_blue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0xff,};
    }

    pub fn light_purple(&self) -> RGB {
        return RGB { r: 0xff, g: 0x00, b: 0xff,};
    }

    pub fn light_pale_blue(&self) -> RGB {
        return RGB { r: 0x00, g: 0xff, b: 0xff,};
    }

    pub fn white(&self) -> RGB {
        return RGB { r: 0xff, g: 0xff, b: 0xff,};
    }

    pub fn light_gray(&self) -> RGB {
        return RGB { r: 0xc6, g: 0xc6, b: 0xc6,};
    }

    pub fn dark_red(&self) -> RGB {
        return RGB { r: 0x84, g: 0x00, b: 0x00,};
    }

    pub fn dark_green(&self) -> RGB {
        return RGB { r: 0x00, g: 0x84, b: 0x00,};
    }

    pub fn dark_yellow(&self) -> RGB {
        return RGB { r: 0x84, g: 0x84, b: 0x00,};
    }

    pub fn dark_blue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x00, b: 0x84,};
    }

    pub fn dark_purple(&self) -> RGB {
        return RGB { r: 0x84, g: 0x00, b: 0x84,};
    }

    pub fn dark_pale_blue(&self) -> RGB {
        return RGB { r: 0x00, g: 0x84, b: 0x84,};
    }

    pub fn dark_gray(&self) -> RGB {
        return RGB { r: 0x84, g: 0x84, b: 0x84,};
    }
}
