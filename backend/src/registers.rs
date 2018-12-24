#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Reg(pub u8);

impl From<Reg> for u32 {
    fn from(reg: Reg) -> u32 {
        reg.0 as u32
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FReg(pub u8);

impl From<FReg> for u32 {
    fn from(reg: FReg) -> u32 {
        reg.0 as u32
    }
}

pub const REG_COUNT: usize = 16;
#[cfg(target_family = "windows")]
pub static REG_PARAMS: [Reg; 4] = [RCX, RDX, R8, R9];

#[cfg(target_family = "unix")]
pub static REG_PARAMS: [Reg; 6] = [RDI, RSI, RDX, RCX, R8, R9];
pub static SCRATCH: [Reg; 3] = [R9, R8, RDI];

pub const REG_RESULT: Reg = RAX;
pub const REG_TMP1: Reg = R10;
pub const REG_TMP2: Reg = R11;
pub const REG_SP: Reg = RSP;
pub const REG_FP: Reg = RBP;
pub const REG_THREAD: Reg = R15;

pub const RAX: Reg = Reg(0);
pub const RCX: Reg = Reg(1);
pub const RDX: Reg = Reg(2);
pub const RBX: Reg = Reg(3);
pub const RSP: Reg = Reg(4);
pub const RBP: Reg = Reg(5);
pub const RSI: Reg = Reg(6);
pub const RDI: Reg = Reg(7);

pub const R8: Reg = Reg(8);
pub const R9: Reg = Reg(9);
pub const R10: Reg = Reg(10);
pub const R11: Reg = Reg(11);
pub const R12: Reg = Reg(12);
pub const R13: Reg = Reg(13);
pub const R14: Reg = Reg(14);
pub const R15: Reg = Reg(15);

pub const RIP: Reg = Reg(16);

pub const FREG_RESULT: FReg = XMM0;
pub const FREG_TMP1: FReg = XMM1;

#[cfg(target_family = "unix")]
pub static FREG_PARAMS: [FReg; 8] = [XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7];

#[cfg(target_family = "windows")]
pub static FREG_PARAMS: [FReg; 4] = [XMM0, XMM1, XMM2, XMM3];

pub const XMM0: FReg = FReg(0);
pub const XMM1: FReg = FReg(1);
pub const XMM2: FReg = FReg(2);
pub const XMM3: FReg = FReg(3);
pub const XMM4: FReg = FReg(4);
pub const XMM5: FReg = FReg(5);
pub const XMM6: FReg = FReg(6);
pub const XMM7: FReg = FReg(7);
pub const XMM8: FReg = FReg(8);
pub const XMM9: FReg = FReg(9);
pub const XMM10: FReg = FReg(10);
pub const XMM11: FReg = FReg(11);
pub const XMM12: FReg = FReg(12);
pub const XMM13: FReg = FReg(13);
pub const XMM14: FReg = FReg(14);
pub const XMM15: FReg = FReg(15);

impl Reg {
    // these four register need sometimes special treatment: e.g. because of bl vs bh
    // for byte operations
    pub fn is_basic_reg(self) -> bool {
        self == RAX || self == RBX || self == RCX || self == RDX
    }

    pub fn int(self) -> u8 {
        assert!(self != RIP);

        self.0
    }

    pub fn msb(self) -> u8 {
        assert!(self != RIP);

        (self.int() >> 3) & 0x01
    }

    pub fn and7(self) -> u8 {
        assert!(self != RIP);

        self.int() & 0x07
    }
}

impl FReg {
    pub fn msb(self) -> u8 {
        (self.0 >> 3) & 0x01
    }

    pub fn and7(self) -> u8 {
        self.0 & 0x07
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Register {
    Float(FReg),
    General(Reg),
}

impl Register {
    pub fn freg(&self) -> FReg {
        if let Register::Float(freg) = self {
            return *freg;
        } else {
            panic!("Expected float register, found general register")
        }
    }

    pub fn reg(&self) -> Reg {
        if let Register::General(reg) = self {
            return *reg;
        } else {
            panic!("Expected general register, found float register")
        }
    }
}
