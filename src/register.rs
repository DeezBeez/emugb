use crate::flags_register::FlagsRegister;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    // --- get set af
    fn get_af(&self) -> u16 {
        let mut af = (self.b as u16) << 8;
        af |= u8::from(&self.f) as u16;
        af
    }
    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0x00FF) as u8);
    }

    // --- get set bc
    fn get_bc(&self) -> u16 {
        let mut bc = (self.b as u16) << 8;
        bc |= self.c as u16;
        bc
    }
    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    // --- get set de
    fn get_de(&self) -> u16 {
        let mut de = (self.d as u16) << 8;
        de |= self.e as u16;
        de
    }
    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    // --- get set hl
    fn get_hl(&self) -> u16 {
        let mut hl = (self.h as u16) << 8;
        hl |= self.l as u16;
        hl
    }
    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}


