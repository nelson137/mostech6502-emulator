// use std::convert::TryFrom;

use bitvec::prelude::*;

use crate::mem::Memory;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Flag {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    Decimal = 3,
    Break = 4,
    Overflow = 5,
    Negative = 6,
}

impl Flag {
    pub fn pos(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
#[allow(non_camel_case_types, dead_code)]
pub enum Ins {
    ADC_IMM   = 0x69,
    ADC_0     = 0x65,
    ADC_0_X   = 0x75,
    ADC_ABS   = 0x6d,
    ADC_ABS_X = 0x7d,
    ADC_ABS_Y = 0x79,
    ADC_IND_X = 0x61,
    ADC_IND_Y = 0x71,

    AND_IMM   = 0x29,
    AND_0     = 0x25,
    AND_0_X   = 0x35,
    AND_ABS   = 0x2d,
    AND_ABS_X = 0x3d,
    AND_ABS_Y = 0x39,
    AND_IND_X = 0x21,
    AND_IND_Y = 0x31,

    ASL_ACC   = 0x0a,
    ASL_0     = 0x06,
    ASL_0_X   = 0x16,
    ASL_ABS   = 0x0e,
    ASL_ABS_X = 0x1e,

    BCC       = 0x90,
    BCS       = 0xb0,
    BEQ       = 0xf0,
    BMI       = 0x30,
    BNE       = 0xd0,
    BPL       = 0x10,

    BIT_0     = 0x24,
    BIT_ABS   = 0x2c,

    BRK       = 0x00,

    BVC       = 0x50,
    BVS       = 0x70,

    CLC       = 0x18,
    CLD       = 0xd8,
    CLI       = 0x58,
    CLV       = 0xb8,

    CMP_IMM   = 0xc9,
    CMP_0     = 0xc5,
    CMP_0_X   = 0xd5,
    CMP_ABS   = 0xcd,
    CMP_ABS_X = 0xdd,
    CMP_ABS_Y = 0xd9,
    CMP_IND_X = 0xc1,
    CMP_IND_Y = 0xd1,

    CPX_IMM   = 0xe0,
    CPX_0     = 0xe4,
    CPX_ABS   = 0xec,

    CPY_IMM   = 0xc0,
    CPY_0     = 0xc4,
    CPY_ABS   = 0xcc,

    DEC_0     = 0xc6,
    DEC_0_X   = 0xd6,
    DEC_ABS   = 0xce,
    DEC_ABS_X = 0xde,

    DEX       = 0xca,
    DEY       = 0x88,

    EOR_IMM   = 0x49,
    EOR_0     = 0x45,
    EOR_0_X   = 0x55,
    EOR_ABS   = 0x4d,
    EOR_ABS_X = 0x5d,
    EOR_ABS_Y = 0x59,
    EOR_IND_X = 0x41,
    EOR_IND_Y = 0x51,

    INC_0     = 0xe6,
    INC_0_X   = 0xf6,
    INC_ABS   = 0xee,
    INC_ABS_X = 0xfe,

    INX       = 0xe8,
    INY       = 0xc8,

    JMP_ABS   = 0x4c,
    JMP_IND   = 0x6c,

    JSR       = 0x20,

    LDA_IMM   = 0xa9,
    LDA_0     = 0xa5,
    LDA_0_X   = 0xb5,
    LDA_ABS   = 0xad,
    LDA_ABS_X = 0xbd,
    LDA_ABS_Y = 0xb9,
    LDA_IND_X = 0xa1,
    LDA_IND_Y = 0xb1,

    LDX_IMM   = 0xa2,
    LDX_0     = 0xa6,
    LDX_0_Y   = 0xb6,
    LDX_ABS   = 0xae,
    LDX_ABS_Y = 0xbe,

    LDY_IMM   = 0xa0,
    LDY_0     = 0xa4,
    LDY_0_X   = 0xb4,
    LDY_ABS   = 0xac,
    LDY_ABS_X = 0xbc,

    LSR_ACC   = 0x4a,
    LSR_0     = 0x46,
    LSR_0_X   = 0x56,
    LSR_ABS   = 0x4e,
    LSR_ABS_X = 0x5e,

    NOP       = 0xea,

    ORA_IMM   = 0x09,
    ORA_0     = 0x05,
    ORA_0_X   = 0x15,
    ORA_ABS   = 0x1d,
    ORA_ABS_X = 0x0d,
    ORA_ABS_Y = 0x19,
    ORA_IND_X = 0x01,
    ORA_IND_Y = 0x11,

    PHA       = 0x48,
    PHP       = 0x08,

    PLA       = 0x68,
    PLP       = 0x28,

    ROL_ACC   = 0x2a,
    ROL_0     = 0x26,
    ROL_0_X   = 0x36,
    ROL_ABS   = 0x2e,
    ROL_ABS_X = 0x3e,

    ROR_ACC   = 0x6a,
    ROR_0     = 0x66,
    ROR_0_X   = 0x76,
    ROR_ABS   = 0x6e,
    ROR_ABS_X = 0x7e,

    RTI       = 0x40,
    RTS       = 0x60,

    SBC_IMM   = 0xe9,
    SBC_0     = 0xe5,
    SBC_0_X   = 0xf5,
    SBC_ABS   = 0xed,
    SBC_ABS_X = 0xfd,
    SBC_ABS_Y = 0xf9,
    SBC_IND_X = 0xe1,
    SBC_IND_Y = 0xf1,

    SEC       = 0x38,
    SED       = 0xf8,
    SEI       = 0x78,

    STA_0     = 0x85,
    STA_0_X   = 0x95,
    STA_ABS   = 0x8d,
    STA_ABS_X = 0x9d,
    STA_ABS_Y = 0x99,
    STA_IND_X = 0x81,
    STA_IND_Y = 0x91,

    STX_0     = 0x86,
    STX_0_Y   = 0x96,
    STX_ABS   = 0x8e,

    STY_0     = 0x84,
    STY_0_X   = 0x94,
    STY_ABS   = 0x8c,

    TAX       = 0xaa,
    TAY       = 0xa8,
    TSX       = 0xba,
    TXA       = 0x8a,
    TXS       = 0x9a,
    TYA       = 0x98,
}

#[macro_export]
macro_rules! ins {
    ($i:tt) => ($i as u8)
}

enum InsArg {
    ArgNone,
    ArgByte(u8),
    ArgWord(u16),
}

impl InsArg {
    fn byte(self) -> Option<u8> {
        if let Self::ArgByte(val) = self {
            Some(val)
        } else {
            None
        }
    }

    fn byte_or(self, err: u8) -> Result<u8, u8> {
        if let Self::ArgByte(val) = self {
            Ok(val)
        } else {
            Err(err)
        }
    }

    fn word(self) -> Option<u16> {
        if let Self::ArgWord(val) = self {
            Some(val)
        } else {
            None
        }
    }

    fn word_or(self, err: u8) -> Result<u16, u8> {
        if let Self::ArgWord(val) = self {
            Ok(val)
        } else {
            Err(err)
        }
    }
}

pub struct Cpu {
    _flags: BitArr!(for 7, in LocalBits, u8),
    _ticks: u128,

    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            _flags: bitarr![LocalBits, u8; 0; 7],
            _ticks: 0,
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn flag_get(&self, flag: Flag) -> u8 {
        *self._flags.get(flag.pos()).unwrap() as u8
    }

    pub fn flag_set(&mut self, flag: Flag) {
        self._flags.set(flag.pos(), true);
    }

    pub fn flag_clear(&mut self, flag: Flag) {
        self._flags.set(flag.pos(), false);
    }

    fn set_flags_zn(&mut self, val: u8) {
        self._flags.set(Flag::Zero.pos(), val == 0);
        self._flags.set(Flag::Negative.pos(), (val as i8) < 0);
    }

    fn set_flags_czn(&mut self, val: u8) {
        if val & 0x80 != 0 {
            self.flag_set(Flag::Carry);
        }
        self.set_flags_zn(val);
    }

    fn get_sp(&self) -> u16 {
        self.sp as u16 | 0x0100
    }

    fn exec_ins(
        &mut self,
        mem: &mut Memory,
        ins: Ins,
        arg: InsArg
    ) -> Result<(), u8> {
        use Flag::*;
        use Ins::*;
        match ins {
            ADC_IMM => {
            }
            ADC_0 => {
            }
            ADC_0_X => {
            }
            ADC_ABS => {
            }
            ADC_ABS_X => {
            }
            ADC_ABS_Y => {
            }
            ADC_IND_X => {
            }
            ADC_IND_Y => {
            }

            AND_IMM => {
                self.a &= arg.byte_or(2)?;
                self.set_flags_zn(self.a);
            }
            AND_0 => {
                self.a &= mem.read_byte(arg.byte_or(2)? as u16);
                self.set_flags_zn(self.a);
            }
            AND_0_X => {
                self.a &= mem.read_byte(arg.byte_or(2)? as u16 + self.x as u16);
                self.set_flags_zn(self.a);
            }
            AND_ABS => {
                self.a &= mem.read_byte(arg.word_or(2)?);
                self.set_flags_zn(self.a);
            }
            AND_ABS_X => {
                self.a &= mem.read_byte(arg.word_or(2)? + self.x as u16);
                self.set_flags_zn(self.a);
            }
            AND_ABS_Y => {
                self.a &= mem.read_byte(arg.word_or(2)? + self.y as u16);
                self.set_flags_zn(self.a);
            }
            AND_IND_X => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.x as u16);
                self.a &= mem.read_byte(addr);
                self.set_flags_zn(self.a);
            }
            AND_IND_Y => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.y as u16);
                self.a &= mem.read_byte(addr);
                self.set_flags_zn(self.a);
            }

            ASL_ACC => {
            }
            ASL_0 => {
            }
            ASL_0_X => {
            }
            ASL_ABS => {
            }
            ASL_ABS_X => {
            }

            BEQ => {
                if self.flag_get(Zero) == 1 {
                    self.pc += arg.byte_or(2)? as u16;
                }
            }
            BNE => {
                if self.flag_get(Zero) == 0 {
                    self.pc += arg.byte_or(2)? as u16;
                }
            }
            BMI => {
                if self.flag_get(Negative) == 1 {
                    self.pc += arg.byte_or(2)? as u16;
                }
            }
            BPL => {
                if self.flag_get(Negative) == 0 {
                    self.pc += arg.byte_or(2)? as u16;
                }
            }
            BCS => {
                if self.flag_get(Carry) == 1 {
                    self.pc += arg.byte_or(2)? as u16;
                }
            }
            BCC => {
                if self.flag_get(Carry) == 0 {
                    self.pc += arg.byte_or(2)? as u16;
                }
            }
            BVS => {
                if self.flag_get(Overflow) == 1 {
                    self.pc += arg.byte_or(2)? as u16;
                }
            }
            BVC => {
                if self.flag_get(Overflow) == 0 {
                    self.pc += arg.byte_or(2)? as u16;
                }
            }

            BIT_0 => {
            }
            BIT_ABS => {
            }

            BRK => {
            }

            CLC => self.flag_clear(Carry),
            CLD => self.flag_clear(Decimal),
            CLI => self.flag_clear(InterruptDisable),
            CLV => self.flag_clear(Overflow),

            CMP_IMM => {
                let result = self.a as i8 - arg.byte_or(2)? as i8;
                self.set_flags_czn(result as u8);
            }
            CMP_0 => {
                let mut result = self.a as i8;
                result -= mem.read_byte(arg.byte_or(2)? as u16) as i8;
                self.set_flags_czn(result as u8);
            }
            CMP_0_X => {
                let mut result = self.a as i8;
                result -= mem.read_byte(arg.byte_or(2)? as u16 + self.x as u16) as i8;
                self.set_flags_czn(result as u8);
            }
            CMP_ABS => {
                let mut result = self.a as i8;
                result -= mem.read_byte(arg.word_or(2)?) as i8;
                self.set_flags_czn(result as u8);
            }
            CMP_ABS_X => {
                let mut result = self.a as i8;
                result -= mem.read_byte(arg.word_or(2)? + self.x as u16) as i8;
                self.set_flags_czn(result as u8);
            }
            CMP_ABS_Y => {
                let mut result = self.a as i8;
                result -= mem.read_byte(arg.word_or(2)? + self.y as u16) as i8;
                self.set_flags_czn(result as u8);
            }
            CMP_IND_X => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.x as u16);
                let mut result = self.a as i8;
                result -= mem.read_byte(addr) as i8;
                self.set_flags_czn(result as u8);
            }
            CMP_IND_Y => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.y as u16);
                let mut result = self.a as i8;
                result -= mem.read_byte(addr) as i8;
                self.set_flags_czn(result as u8);
            }

            CPX_IMM => {
                let result = self.x as i8 - arg.byte_or(2)? as i8;
                self.set_flags_czn(result as u8);
            }
            CPX_0 => {
                let mut result = self.x as i8;
                result -= mem.read_byte(arg.byte_or(2)? as u16) as i8;
                self.set_flags_czn(result as u8);
            }
            CPX_ABS => {
                let mut result = self.x as i8;
                result -= mem.read_byte(arg.word_or(2)?) as i8;
                self.set_flags_czn(result as u8);
            }

            CPY_IMM => {
                let result = self.y as i8 - arg.byte_or(2)? as i8;
                self.set_flags_czn(result as u8);
            }
            CPY_0 => {
                let mut result = self.y as i8;
                result -= mem.read_byte(arg.byte_or(2)? as u16) as i8;
                self.set_flags_czn(result as u8);
            }
            CPY_ABS => {
                let mut result = self.y as i8;
                result -= mem.read_byte(arg.word_or(2)?) as i8;
                self.set_flags_czn(result as u8);
            }

            DEC_0 => {
                let addr = arg.byte_or(2)? as u16;
                let value = mem.read_byte(addr) - 1;
                mem.write_bytes(addr, &[value]);
                self.set_flags_zn(value);
            }
            DEC_0_X => {
                let addr = arg.byte_or(2)? as u16 + self.x as u16;
                let value = mem.read_byte(addr) - 1;
                mem.write_bytes(addr, &[value]);
                self.set_flags_zn(value);
            }
            DEC_ABS => {
                let addr = arg.word_or(2)?;
                let value = mem.read_byte(addr) - 1;
                mem.write_bytes(addr, &[value]);
                self.set_flags_zn(value);
            }
            DEC_ABS_X => {
                let addr = arg.word_or(2)? + self.x as u16;
                let value = mem.read_byte(addr) - 1;
                mem.write_bytes(addr, &[value]);
                self.set_flags_zn(value);
            }

            DEX => {
                self.x -= 1;
                self.set_flags_zn(self.x);
            }
            DEY => {
                self.y -= 1;
                self.set_flags_zn(self.y);
            }

            EOR_IMM => {
                self.a ^= arg.byte_or(2)?;
                self.set_flags_zn(self.a);
            }
            EOR_0 => {
                self.a ^= mem.read_byte(arg.byte_or(2)? as u16);
                self.set_flags_zn(self.a);
            }
            EOR_0_X => {
                self.a ^= mem.read_byte(arg.byte_or(2)? as u16 + self.x as u16);
                self.set_flags_zn(self.a);
            }
            EOR_ABS => {
                self.a ^= mem.read_byte(arg.word_or(2)?);
                self.set_flags_zn(self.a);
            }
            EOR_ABS_X => {
                self.a ^= mem.read_byte(arg.word_or(2)? + self.x as u16);
                self.set_flags_zn(self.a);
            }
            EOR_ABS_Y => {
                self.a ^= mem.read_byte(arg.word_or(2)? + self.y as u16);
                self.set_flags_zn(self.a);
            }
            EOR_IND_X => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.x as u16);
                self.a ^= mem.read_byte(addr);
                self.set_flags_zn(self.a);
            }
            EOR_IND_Y => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.y as u16);
                self.a ^= mem.read_byte(addr);
                self.set_flags_zn(self.a);
            }

            INC_0 => {
                let addr = arg.byte_or(2)? as u16;
                let value = mem.read_byte(addr) + 1;
                mem.write_bytes(addr, &[value]);
                self.set_flags_zn(value);
            }
            INC_0_X => {
                let addr = arg.byte_or(2)? as u16 + self.x as u16;
                let value = mem.read_byte(addr) + 1;
                mem.write_bytes(addr, &[value]);
                self.set_flags_zn(value);
            }
            INC_ABS => {
                let addr = arg.word_or(2)?;
                let value = mem.read_byte(addr) + 1;
                mem.write_bytes(addr, &[value]);
                self.set_flags_zn(value);
            }
            INC_ABS_X => {
                let addr = arg.word_or(2)? + self.x as u16;
                let value = mem.read_byte(addr) + 1;
                mem.write_bytes(addr, &[value]);
                self.set_flags_zn(value);
            }

            INX => {
                self.x += 1;
                self.set_flags_zn(self.x);
            }
            INY => {
                self.y += 1;
                self.set_flags_zn(self.y);
            }

            JMP_ABS => {
                self.pc = arg.word_or(2)?;
            }
            JMP_IND => {
                self.pc = mem.read_word(arg.word_or(2)?);
            }

            JSR => {
                self.sp -= 1;
                mem.write_bytes(self.get_sp(), &[
                    (self.pc & 0x00ff) as u8,
                    ((self.pc & 0xff00) >> 8) as u8,
                ]);
                self.sp -= 1;
                self.pc = arg.word_or(2)?;
            }

            LDA_IMM => {
                self.a = arg.byte_or(2)?;
                self.set_flags_zn(self.a);
            }
            LDA_0 => {
                self.a = mem.read_byte(arg.byte_or(2)? as u16);
                self.set_flags_zn(self.a);
            }
            LDA_0_X => {
                self.a = mem.read_byte(arg.byte_or(2)? as u16 + self.x as u16);
                self.set_flags_zn(self.a);
            }
            LDA_ABS => {
                self.a = mem.read_byte(arg.word_or(2)?);
                self.set_flags_zn(self.a);
            }
            LDA_ABS_X => {
                self.a = mem.read_byte(arg.word_or(2)? + self.x as u16);
                self.set_flags_zn(self.a);
            }
            LDA_ABS_Y => {
                self.a = mem.read_byte(arg.word_or(2)? + self.y as u16);
                self.set_flags_zn(self.a);
            }
            LDA_IND_X => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.x as u16);
                self.a = mem.read_byte(addr);
                self.set_flags_zn(self.a);
            }
            LDA_IND_Y => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.y as u16);
                self.a = mem.read_byte(addr);
                self.set_flags_zn(self.a);
            }

            LDX_IMM => {
                self.x = arg.byte_or(2)?;
                self.set_flags_zn(self.x);
            }
            LDX_0 => {
                self.x = mem.read_byte(arg.byte_or(2)? as u16);
                self.set_flags_zn(self.x);
            }
            LDX_0_Y => {
                self.x = mem.read_byte(arg.byte_or(2)? as u16 + self.y as u16);
                self.set_flags_zn(self.x);
            }
            LDX_ABS => {
                self.x = mem.read_byte(arg.word_or(2)?);
                self.set_flags_zn(self.x);
            }
            LDX_ABS_Y => {
                self.x = mem.read_byte(arg.word_or(2)? + self.y as u16);
                self.set_flags_zn(self.x);
            }

            LDY_IMM => {
                self.y = arg.byte_or(2)?;
                self.set_flags_zn(self.y);
            }
            LDY_0 => {
                self.y = mem.read_byte(arg.byte_or(2)? as u16);
                self.set_flags_zn(self.y);
            }
            LDY_0_X => {
                self.y = mem.read_byte(arg.byte_or(2)? as u16 + self.x as u16);
                self.set_flags_zn(self.y);
            }
            LDY_ABS => {
                self.y = mem.read_byte(arg.word_or(2)?);
                self.set_flags_zn(self.y);
            }
            LDY_ABS_X => {
                self.y = mem.read_byte(arg.word_or(2)? + self.y as u16);
                self.set_flags_zn(self.y);
            }

            LSR_ACC => {
            }
            LSR_0 => {
            }
            LSR_0_X => {
            }
            LSR_ABS => {
            }
            LSR_ABS_X => {
            }

            NOP => (),

            ORA_IMM => {
                self.a |= arg.byte_or(2)?;
                self.set_flags_zn(self.a);
            }
            ORA_0 => {
                self.a |= mem.read_byte(arg.byte_or(2)? as u16);
                self.set_flags_zn(self.a);
            }
            ORA_0_X => {
                self.a |= mem.read_byte(arg.byte_or(2)? as u16 + self.x as u16);
                self.set_flags_zn(self.a);
            }
            ORA_ABS => {
                self.a |= mem.read_byte(arg.word_or(2)?);
                self.set_flags_zn(self.a);
            }
            ORA_ABS_X => {
                self.a |= mem.read_byte(arg.word_or(2)? + self.x as u16);
                self.set_flags_zn(self.a);
            }
            ORA_ABS_Y => {
                self.a |= mem.read_byte(arg.word_or(2)? + self.y as u16);
                self.set_flags_zn(self.a);
            }
            ORA_IND_X => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.x as u16);
                self.a |= mem.read_byte(addr);
                self.set_flags_zn(self.a);
            }
            ORA_IND_Y => {
                let addr = mem.read_word(arg.byte_or(2)? as u16 + self.y as u16);
                self.a |= mem.read_byte(addr);
                self.set_flags_zn(self.a);
            }

            PHA => {
                mem.write_bytes(self.get_sp(), &[self.a]);
                self.sp -= 1;
            }
            PHP => {
                mem.write_bytes(self.get_sp(), &[self._flags.as_buffer()[0]]);
                self.sp -= 1;
            }

            PLA => {
                self.sp += 1;
                self.a = mem.read_byte(self.get_sp());
            }
            PLP => {
                self.sp += 1;
                self._flags.as_mut_buffer()[0] = mem.read_byte(self.get_sp());
            }

            ROL_ACC => {
            }
            ROL_0 => {
            }
            ROL_0_X => {
            }
            ROL_ABS => {
            }
            ROL_ABS_X => {
            }

            ROR_ACC => {
            }
            ROR_0 => {
            }
            ROR_0_X => {
            }
            ROR_ABS => {
            }
            ROR_ABS_X => {
            }

            RTI => {
                self.sp += 1;
                self._flags.as_mut_buffer()[0] = mem.read_byte(self.get_sp());
                self.sp += 1;
                self.pc = mem.read_word(self.get_sp());
            }
            RTS => {
                self.sp += 1;
                self.pc = mem.read_word(self.get_sp());
            }

            SBC_IMM => {
            }
            SBC_0 => {
            }
            SBC_0_X => {
            }
            SBC_ABS => {
            }
            SBC_ABS_X => {
            }
            SBC_ABS_Y => {
            }
            SBC_IND_X => {
            }
            SBC_IND_Y => {
            }

            SEC => self.flag_set(Carry),
            SED => self.flag_set(Decimal),
            SEI => self.flag_set(InterruptDisable),

            STA_0     => mem.write_bytes(arg.byte_or(2)? as u16, &[self.a]),
            STA_0_X   => mem.write_bytes(arg.byte_or(2)? as u16 + self.x as u16, &[self.a]),
            STA_ABS   => mem.write_bytes(arg.word_or(2)?, &[self.a]),
            STA_ABS_X => mem.write_bytes(arg.word_or(2)? + self.x as u16, &[self.a]),
            STA_ABS_Y => mem.write_bytes(arg.word_or(2)? + self.y as u16, &[self.a]),
            STA_IND_X => mem.write_bytes(mem.read_word(arg.byte_or(2)? as u16 + self.x as u16), &[self.a]),
            STA_IND_Y => mem.write_bytes(mem.read_word(arg.byte_or(2)? as u16 + self.y as u16), &[self.a]),

            STX_0   => mem.write_bytes(arg.byte_or(2)? as u16, &[self.x]),
            STX_0_Y => mem.write_bytes(arg.byte_or(2)? as u16 + self.y as u16, &[self.x]),
            STX_ABS => mem.write_bytes(arg.word_or(2)?, &[self.x]),

            STY_0   => mem.write_bytes(arg.byte_or(2)? as u16, &[self.y]),
            STY_0_X => mem.write_bytes(arg.byte_or(2)? as u16 + self.x as u16, &[self.y]),
            STY_ABS => mem.write_bytes(arg.word_or(2)?, &[self.y]),

            TAX => {
                self.x = self.a;
                self.set_flags_zn(self.x);
            }
            TAY => {
                self.y = self.a;
                self.set_flags_zn(self.y);
            }
            TSX => {
                self.x = self.sp;
                self.set_flags_zn(self.x);
            }
            TXA => {
                self.a = self.x;
                self.set_flags_zn(self.a);
            }
            TXS => {
                self.sp = self.x;
                self.set_flags_zn(self.sp);
            }
            TYA => {
                self.a = self.y;
                self.set_flags_zn(self.a);
            }
        }
        Ok(())
    }

    pub fn execute(&mut self, mem: &mut Memory) -> Result<(), u8> {
        // use Ins::*;
        // use InsArg::*;
        self.pc = mem.read_word(0xfffc);
        // mem.read_byte(self.pc).into()  // get ins
        Ok(())
    }

    pub fn reset(&mut self, mem: &mut Memory) -> Result<(), u8> {
        Ok(())
    }
}
