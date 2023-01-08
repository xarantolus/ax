use crate::fatal_error;
use crate::instructions::operand::Operand;
use iced_x86::Instruction;

use super::{axecutor::Axecutor, errors::AxError, registers::SupportedRegister};

pub(crate) const NO_WRITEBACK: u64 = 0x8000_0000_0000_0000;

// Functions used by the calculate_rm_r macro
impl Axecutor {
    pub(crate) fn calculate_rm_r_8f(
        &mut self,
        i: Instruction,
        op: impl Fn(u8, u8) -> (u8, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_8(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_8(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u8, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u8(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_8(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_8(r)?;
                let (result, flags) = op(dest_val as u8, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u8(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_8(r, result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_16f(
        &mut self,
        i: Instruction,
        op: impl Fn(u16, u16) -> (u16, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_16(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_16(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u16, src_val as u16);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_16(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_16(r)?;
                let (result, flags) = op(dest_val as u16, src_val as u16);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_16(r, result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_32f(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u32) -> (u32, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_32(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_32(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u32, src_val as u32);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_32(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_32(r)?;
                let (result, flags) = op(dest_val as u32, src_val as u32);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_32(r, result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_64f(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u64) -> (u64, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_64(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_64(self.mem_addr(m))?;
                let (result, flags) = op(dest_val, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_64(self.mem_addr(m), result)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_64(r)?;
                let (result, flags) = op(dest_val, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_64(r, result)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_16f_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u16, u8) -> (u16, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_8(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_16(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u16, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_16(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_16(r)?;
                let (result, flags) = op(dest_val as u16, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_16(r, result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_32f_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u8) -> (u32, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_8(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_32(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u32, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_32(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_32(r)?;
                let (result, flags) = op(dest_val as u32, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_32(r, result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_64f_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u8) -> (u64, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_8(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_64(self.mem_addr(m))?;
                let (result, flags) = op(dest_val, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_64(self.mem_addr(m), result)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_64(r)?;
                let (result, flags) = op(dest_val, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_64(r, result)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u8, u8) -> u8,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_8(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_8(self.mem_addr(m))?;
                let result = op(dest_val as u8, src_val as u8);
                self.set_flags_u8(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_8(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_8(r)?;
                let result = op(dest_val as u8, src_val as u8);
                self.set_flags_u8(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_8(r, result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_16(
        &mut self,
        i: Instruction,
        op: impl Fn(u16, u16) -> u16,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_16(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_16(self.mem_addr(m))?;
                let result = op(dest_val as u16, src_val as u16);
                self.set_flags_u16(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_16(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_16(r)?;
                let result = op(dest_val as u16, src_val as u16);
                self.set_flags_u16(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_16(r, result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_32(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u32) -> u32,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_32(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_32(self.mem_addr(m))?;
                let result = op(dest_val as u32, src_val as u32);
                self.set_flags_u32(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_32(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_32(r)?;
                let result = op(dest_val as u32, src_val as u32);
                self.set_flags_u32(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_32(r, result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_r_64(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u64) -> u64,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = self.reg_read_64(src.into())?;

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_64(self.mem_addr(m))?;
                let result = op(dest_val, src_val);
                self.set_flags_u64(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_64(self.mem_addr(m), result)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_64(r)?;
                let result = op(dest_val, src_val);
                self.set_flags_u64(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_64(r, result)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid destination operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }
}

#[macro_export]
macro_rules! calculate_rm_r {
    [u8f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_8f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u16f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_16f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_32f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_64f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u16f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_16f_8($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_32f_8($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_64f_8($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_8($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u16; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_16($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_32($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_r_64($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u8; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_r![u8; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_r![u16; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_r![u32; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_r![u64; $self; $i; $op; (set: 0; clear: 0)]
    };
}

// Functions used by the calculate_r_rm macro
impl Axecutor {
    pub(crate) fn calculate_r_rm_8f(
        &mut self,
        i: Instruction,
        op: impl Fn(u8, u8) -> (u8, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let dest_reg: SupportedRegister = dest.into();
        let dest_val = self.reg_read_8(dest_reg)?;

        match src {
            Operand::Memory(m) => {
                let src_val = self.mem_read_8(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u8, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u8(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_8(dest_reg, result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let src_val = self.reg_read_8(r)?;
                let (result, flags) = op(dest_val as u8, src_val as u8);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u8(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_8(dest_reg, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                src,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_r_rm_16f(
        &mut self,
        i: Instruction,
        op: impl Fn(u16, u16) -> (u16, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let dest_reg: SupportedRegister = dest.into();
        let dest_val = self.reg_read_16(dest_reg)?;

        match src {
            Operand::Memory(m) => {
                let src_val = self.mem_read_16(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u16, src_val as u16);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_16(dest_reg, result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let src_val = self.reg_read_16(r)?;
                let (result, flags) = op(dest_val as u16, src_val as u16);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_16(dest_reg, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                src,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_r_rm_32f(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u32) -> (u32, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let dest_reg: SupportedRegister = dest.into();
        let dest_val = self.reg_read_32(dest_reg)?;

        match src {
            Operand::Memory(m) => {
                let src_val = self.mem_read_32(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u32, src_val as u32);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_32(dest_reg, result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let src_val = self.reg_read_32(r)?;
                let (result, flags) = op(dest_val as u32, src_val as u32);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_32(dest_reg, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                src,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_r_rm_64f(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u64) -> (u64, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let dest_reg: SupportedRegister = dest.into();
        let dest_val = self.reg_read_64(dest_reg)?;

        match src {
            Operand::Memory(m) => {
                let src_val = self.mem_read_64(self.mem_addr(m))?;
                let (result, flags) = op(dest_val, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_64(dest_reg, result)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let src_val = self.reg_read_64(r)?;
                let (result, flags) = op(dest_val, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_64(dest_reg, result)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                src,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_r_rm_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u8, u8) -> u8,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Memory(m) => self.mem_read_8(self.mem_addr(m))?,
            Operand::Register(r) => self.reg_read_8(r)?,
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        let dest = dest.into();
        let dest_val = self.reg_read_8(dest)?;
        let result = op(dest_val as u8, src_val as u8);
        self.set_flags_u8(flags_to_set, flags_to_clear, result);
        if (flags_to_set & NO_WRITEBACK) == 0 {
            self.reg_write_8(dest, result as u64)?;
        }
        Ok(())
    }

    pub(crate) fn calculate_r_rm_16(
        &mut self,
        i: Instruction,
        op: impl Fn(u16, u16) -> u16,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            Operand::Register(r) => self.reg_read_16(r)?,
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        let dest = dest.into();
        let dest_val = self.reg_read_16(dest)?;
        let result = op(dest_val as u16, src_val as u16);
        self.set_flags_u16(flags_to_set, flags_to_clear, result);
        if (flags_to_set & NO_WRITEBACK) == 0 {
            self.reg_write_16(dest, result as u64)?;
        }
        Ok(())
    }

    pub(crate) fn calculate_r_rm_32(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u32) -> u32,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            Operand::Register(r) => self.reg_read_32(r)?,
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        let dest = dest.into();
        let dest_val = self.reg_read_32(dest)?;
        let result = op(dest_val as u32, src_val as u32);
        self.set_flags_u32(flags_to_set, flags_to_clear, result);
        if (flags_to_set & NO_WRITEBACK) == 0 {
            self.reg_write_32(dest, result as u64)?;
        }
        Ok(())
    }

    pub(crate) fn calculate_r_rm_64(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u64) -> u64,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Memory(m) => self.mem_read_64(self.mem_addr(m))?,
            Operand::Register(r) => self.reg_read_64(r)?,
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        let dest = dest.into();
        let dest_val = self.reg_read_64(dest)?;
        let result = op(dest_val, src_val);
        self.set_flags_u64(flags_to_set, flags_to_clear, result);
        if (flags_to_set & NO_WRITEBACK) == 0 {
            self.reg_write_64(dest, result)?;
        }
        Ok(())
    }

    pub(crate) fn calculate_r_rm_64_32(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u32) -> u64,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Memory(m) => self.mem_read_32(self.mem_addr(m))?,
            Operand::Register(r) => self.reg_read_32(r)?,
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        let dest = dest.into();
        let dest_val = self.reg_read_64(dest)?;
        let result = op(dest_val, src_val as u32);
        self.set_flags_u64(flags_to_set, flags_to_clear, result);
        if (flags_to_set & NO_WRITEBACK) == 0 {
            self.reg_write_64(dest, result)?;
        }
        Ok(())
    }

    pub(crate) fn calculate_r_rm_32_16(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u16) -> u32,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            Operand::Register(r) => self.reg_read_16(r)?,
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        let dest = dest.into();
        let dest_val = self.reg_read_32(dest)?;
        let result = op(dest_val as u32, src_val as u16);
        self.set_flags_u32(flags_to_set, flags_to_clear, result);
        if (flags_to_set & NO_WRITEBACK) == 0 {
            self.reg_write_32(dest, result as u64)?;
        }
        Ok(())
    }

    pub(crate) fn calculate_r_rm_64_16(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u16) -> u64,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Memory(m) => self.mem_read_16(self.mem_addr(m))?,
            Operand::Register(r) => self.reg_read_16(r)?,
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        let dest = dest.into();
        let dest_val = self.reg_read_64(dest)?;
        let result = op(dest_val, src_val as u16);
        self.set_flags_u64(flags_to_set, flags_to_clear, result);
        if (flags_to_set & NO_WRITEBACK) == 0 {
            self.reg_write_64(dest, result)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! calculate_r_rm {
    [u8f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_8f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u16f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_16f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_32f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_64f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_8($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u16; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_16($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_32($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64; $self:expr; $i:expr; $op:expr;  (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_64($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u8; $self:expr; $i:expr; $op:expr] => {
        calculate_r_rm![u8; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16; $self:expr; $i:expr; $op:expr] => {
        calculate_r_rm![u16; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32; $self:expr; $i:expr; $op:expr] => {
        calculate_r_rm![u32; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32; u16; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_32_16($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64; $self:expr; $i:expr; $op:expr] => {
        calculate_r_rm![u64; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64; u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_64_32($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64; u16; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_r_rm_64_16($i, $op, $flags_to_set, $flags_to_clear)
    };
}

// Functions used by the calculate_rm_imm macro
impl Axecutor {
    pub(crate) fn calculate_rm_imm_8f(
        &mut self,
        i: Instruction,
        op: impl Fn(u8, u8) -> (u8, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    1,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u8
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_8(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u8, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u8(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_8(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_8(r)?;
                let (result, flags) = op(dest_val as u8, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u8(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_8(r, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_16f(
        &mut self,
        i: Instruction,
        op: impl Fn(u16, u16) -> (u16, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    2,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u16
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_16(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u16, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_16(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_16(r)?;
                let (result, flags) = op(dest_val as u16, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_16(r, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_32f(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u32) -> (u32, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    4,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u32
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_32(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u32, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_32(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_32(r)?;
                let (result, flags) = op(dest_val as u32, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_32(r, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_64f(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u64) -> (u64, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    8,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_64(self.mem_addr(m))?;
                let (result, flags) = op(dest_val, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_64(self.mem_addr(m), result)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_64(r)?;
                let (result, flags) = op(dest_val, src_val);
                debug_assert!(flags & NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_64(r, result)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u8, u8) -> u8,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    1,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u8
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_8(self.mem_addr(m))?;
                let result = op(dest_val as u8, src_val);
                self.set_flags_u8(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_8(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_8(r)?;
                let result = op(dest_val as u8, src_val);
                self.set_flags_u8(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_8(r, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_16(
        &mut self,
        i: Instruction,
        op: impl Fn(u16, u16) -> u16,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    2,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u16
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_16(self.mem_addr(m))?;
                let result = op(dest_val as u16, src_val);
                self.set_flags_u16(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_16(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_16(r)?;
                let result = op(dest_val as u16, src_val);
                self.set_flags_u16(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_16(r, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_32(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u32) -> u32,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    4,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u32
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_32(self.mem_addr(m))?;
                let result = op(dest_val as u32, src_val);
                self.set_flags_u32(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_32(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_32(r)?;
                let result = op(dest_val as u32, src_val);
                self.set_flags_u32(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_32(r, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_64(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u64) -> u64,
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    8,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_64(self.mem_addr(m))?;
                let result = op(dest_val, src_val);
                self.set_flags_u64(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.mem_write_64(self.mem_addr(m), result)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_64(r)?;
                let result = op(dest_val, src_val);
                self.set_flags_u64(flags_to_set, flags_to_clear, result);
                if (flags_to_set & NO_WRITEBACK) == 0 {
                    self.reg_write_64(r, result)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_16f_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u16, u8) -> (u16, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    1,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u8
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_16(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u16, src_val);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.mem_write_16(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_16(r)?;
                let (result, flags) = op(dest_val as u16, src_val);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.reg_write_16(r, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_32f_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u32, u8) -> (u32, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    1,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u8
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_32(self.mem_addr(m))?;
                let (result, flags) = op(dest_val as u32, src_val);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.mem_write_32(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_32(r)?;
                let (result, flags) = op(dest_val as u32, src_val);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.reg_write_32(r, result as u64)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }

    pub(crate) fn calculate_rm_imm_64f_8(
        &mut self,
        i: Instruction,
        op: impl Fn(u64, u8) -> (u64, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let (dest, src) = self.instruction_operands_2(i)?;
        let src_val = match src {
            Operand::Immediate { size, data } => {
                debug_assert_eq!(
                    size,
                    1,
                    "Invalid immediate size for {:?} instruction",
                    i.mnemonic()
                );
                data as u8
            }
            _ => fatal_error!(
                "Invalid source operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        };

        match dest {
            Operand::Memory(m) => {
                let dest_val = self.mem_read_64(self.mem_addr(m))?;
                let (result, flags) = op(dest_val, src_val);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.mem_write_64(self.mem_addr(m), result)?;
                }
                Ok(())
            }
            Operand::Register(r) => {
                let dest_val = self.reg_read_64(r)?;
                let (result, flags) = op(dest_val, src_val);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.reg_write_64(r, result)?;
                }
                Ok(())
            }
            _ => fatal_error!(
                "Invalid destination operand {:?} for {:?} instruction",
                dest,
                i.mnemonic()
            ),
        }
    }
}

#[macro_export]
macro_rules! calculate_rm_imm {
    [u8f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_8f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u16f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_16f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_32f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_64f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_8($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u16; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_16($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_32($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_64($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u8; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u8; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u16; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u32; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u64; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u8f; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u8f; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16f; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u16f; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u32f; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u32f; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u64f; $self:expr; $i:expr; $op:expr] => {
        calculate_rm_imm![u64f; $self; $i; $op; (set: 0; clear: 0)]
    };
    [u16f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_16f_8($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_32f_8($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64f; u8; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_imm_64f_8($i, $op, $flags_to_set, $flags_to_clear)
    };
}

// Functions used by the calculate_rm macro
impl Axecutor {
    pub(crate) fn calculate_rm_8f(
        &mut self,
        i: Instruction,
        op: impl Fn(u8) -> (u8, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let dest = self.instruction_operand(i, 0)?;
        match dest {
            Operand::Register(r) => {
                let src_val = self.reg_read_8(r)?;
                let (result, flags) = op(src_val as u8);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u8(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.reg_write_8(r, result as u64)?;
                }
                Ok(())
            }
            Operand::Memory(m) => {
                let src_val = self.mem_read_8(self.mem_addr(m))?;
                let (result, flags) = op(src_val as u8);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u8(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.mem_write_8(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid source operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_16f(
        &mut self,
        i: Instruction,
        op: impl Fn(u16) -> (u16, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let dest = self.instruction_operand(i, 0)?;
        match dest {
            Operand::Register(r) => {
                let src_val = self.reg_read_16(r)?;
                let (result, flags) = op(src_val as u16);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.reg_write_16(r, result as u64)?;
                }
                Ok(())
            }
            Operand::Memory(m) => {
                let src_val = self.mem_read_16(self.mem_addr(m))?;
                let (result, flags) = op(src_val as u16);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u16(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.mem_write_16(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid source operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_32f(
        &mut self,
        i: Instruction,
        op: impl Fn(u32) -> (u32, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let dest = self.instruction_operand(i, 0)?;
        match dest {
            Operand::Register(r) => {
                let src_val = self.reg_read_32(r)?;
                let (result, flags) = op(src_val as u32);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.reg_write_32(r, result as u64)?;
                }
                Ok(())
            }
            Operand::Memory(m) => {
                let src_val = self.mem_read_32(self.mem_addr(m))?;
                let (result, flags) = op(src_val as u32);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u32(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.mem_write_32(self.mem_addr(m), result as u64)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid source operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }

    pub(crate) fn calculate_rm_64f(
        &mut self,
        i: Instruction,
        op: impl Fn(u64) -> (u64, u64),
        flags_to_set: u64,
        flags_to_clear: u64,
    ) -> Result<(), AxError> {
        let dest = self.instruction_operand(i, 0)?;
        match dest {
            Operand::Register(r) => {
                let src_val = self.reg_read_64(r)?;
                let (result, flags) = op(src_val);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.reg_write_64(r, result)?;
                }
                Ok(())
            }
            Operand::Memory(m) => {
                let src_val = self.mem_read_64(self.mem_addr(m))?;
                let (result, flags) = op(src_val);
                debug_assert!(flags & crate::instructions::macros::NO_WRITEBACK == 0, "NO_WRITEBACK flag must not be returned by operation lambda, set it as $flags_to_set");
                self.set_flags_u64(flags_to_set | flags, flags_to_clear, result);
                if (flags_to_set & crate::instructions::macros::NO_WRITEBACK) == 0 {
                    self.mem_write_64(self.mem_addr(m), result)?;
                }
                Ok(())
            }
            _ => {
                fatal_error!(
                    "Invalid source operand {:?} for {:?} instruction",
                    dest,
                    i.mnemonic()
                )
            }
        }
    }
}

#[macro_export]
macro_rules! calculate_rm {
    [u8f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_8f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u16f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_16f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u32f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_32f($i, $op, $flags_to_set, $flags_to_clear)
    };
    [u64f; $self:expr; $i:expr; $op:expr; (set: $flags_to_set:expr; clear: $flags_to_clear:expr)] => {
        $self.calculate_rm_64f($i, $op, $flags_to_set, $flags_to_clear)
    };
}

#[macro_export]
macro_rules! fatal_error {
    ($message:expr, $($arg:tt)*) => {{
        #[cfg(all(target_arch = "wasm32", not(test)))]
        {
            // In WASM we don't panic, as it's not possible to catch panics from JS
            return Err(AxError::from(format!($message, $($arg)*)));
        }

        #[cfg(not(all(target_arch = "wasm32", not(test))))]
        {
            panic!($message, $($arg)*);
        }
    }};
    ($message:expr) => {{
        #[cfg(all(target_arch = "wasm32", not(test)))]
        {
            // In WASM we don't panic, as it's not possible to catch panics from JS
            return Err(AxError::from($message));
        }

        #[cfg(not(all(target_arch = "wasm32", not(test))))]
        {
            panic!($message);
        }
    }};
}

#[macro_export]
macro_rules! assert_fatal {
    ($cond:expr, $message:expr, $($arg:tt)*) => {{
        if !($cond) {
            $crate::fatal_error!($message, $($arg)*);
        }
    }};
    ($cond:expr, $message:expr) => {{
        if !($cond) {
            $crate::fatal_error!($message);
        }
    }};
}

#[macro_export]
macro_rules! opcode_unimplemented {
    ($message:expr) => {{
        #[cfg(target_arch = "wasm32")]
        {
            // In WASM we don't panic, as it's not possible to catch panics from JS
            return Err(AxError::from(format!(
                "Executed unimplemented opcode: {}",
                $message
            )));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            panic!("Executed unimplemented opcode: {}", $message);
        }
    }};
}
