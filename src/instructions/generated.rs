// THIS FILE IS AUTOGENERATED, DO NOT EDIT
// You can regenerate it using `make switch` after creating a new instruction file with `python3 generate.py <mnemonic>`
use crate::helpers::macros::fatal_error;
use std::convert::TryFrom;

use crate::{axecutor::Axecutor, helpers::errors::AxError};
use iced_x86::{
    Instruction,
    Mnemonic::{self, *},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

impl Axecutor {
    pub fn switch_instruction_mnemonic(&mut self, i: Instruction) -> Result<(), AxError> {
        match i.mnemonic() {
            Adc => self.mnemonic_adc(i),
            Add => self.mnemonic_add(i),
            And => self.mnemonic_and(i),
            Call => self.mnemonic_call(i),
            Cdq => self.mnemonic_cdq(i),
            Cdqe => self.mnemonic_cdqe(i),
            Cld => self.mnemonic_cld(i),
            Cmovae => self.mnemonic_cmovae(i),
            Cmove => self.mnemonic_cmove(i),
            Cmovne => self.mnemonic_cmovne(i),
            Cmp => self.mnemonic_cmp(i),
            Cpuid => self.mnemonic_cpuid(i),
            Cqo => self.mnemonic_cqo(i),
            Cwd => self.mnemonic_cwd(i),
            Dec => self.mnemonic_dec(i),
            Div => self.mnemonic_div(i),
            Endbr64 => self.mnemonic_endbr64(i),
            Idiv => self.mnemonic_idiv(i),
            Imul => self.mnemonic_imul(i),
            Inc => self.mnemonic_inc(i),
            Int => self.mnemonic_int(i),
            Int1 => self.mnemonic_int1(i),
            Ja => self.mnemonic_ja(i),
            Jae => self.mnemonic_jae(i),
            Jb => self.mnemonic_jb(i),
            Jbe => self.mnemonic_jbe(i),
            Je => self.mnemonic_je(i),
            Jecxz => self.mnemonic_jecxz(i),
            Jg => self.mnemonic_jg(i),
            Jge => self.mnemonic_jge(i),
            Jl => self.mnemonic_jl(i),
            Jle => self.mnemonic_jle(i),
            Jmp => self.mnemonic_jmp(i),
            Jne => self.mnemonic_jne(i),
            Jno => self.mnemonic_jno(i),
            Jnp => self.mnemonic_jnp(i),
            Jns => self.mnemonic_jns(i),
            Jo => self.mnemonic_jo(i),
            Jp => self.mnemonic_jp(i),
            Jrcxz => self.mnemonic_jrcxz(i),
            Js => self.mnemonic_js(i),
            Lea => self.mnemonic_lea(i),
            Mov => self.mnemonic_mov(i),
            Movsxd => self.mnemonic_movsxd(i),
            Movups => self.mnemonic_movups(i),
            Movzx => self.mnemonic_movzx(i),
            Mul => self.mnemonic_mul(i),
            Neg => self.mnemonic_neg(i),
            Nop => self.mnemonic_nop(i),
            Not => self.mnemonic_not(i),
            Pop => self.mnemonic_pop(i),
            Push => self.mnemonic_push(i),
            Ret => self.mnemonic_ret(i),
            Setb => self.mnemonic_setb(i),
            Sete => self.mnemonic_sete(i),
            Setne => self.mnemonic_setne(i),
            Shl => self.mnemonic_shl(i),
            Shr => self.mnemonic_shr(i),
            Sub => self.mnemonic_sub(i),
            Syscall => self.mnemonic_syscall(i),
            Test => self.mnemonic_test(i),
            Xor => self.mnemonic_xor(i),
            Xorps => self.mnemonic_xorps(i),
            Int3 => self.mnemonic_int3(i),
            _ => Err(AxError::from(format!(
                "cannot execute unimplemented mnemonic {:?}",
                i.mnemonic()
            ))),
        }
    }
}

#[wasm_bindgen(js_name = Mnemonic)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// All mnemonics supported by the emulator
pub enum SupportedMnemonic {
    Adc = 5,
    Add = 7,
    And = 21,
    Call = 59,
    Cdq = 61,
    Cdqe = 62,
    Cld = 66,
    Cmovae = 78,
    Cmove = 81,
    Cmovne = 86,
    Cmp = 93,
    Cpuid = 106,
    Cqo = 107,
    Cwd = 131,
    Dec = 137,
    Div = 138,
    Endbr64 = 152,
    Idiv = 276,
    Imul = 277,
    Inc = 279,
    Int = 287,
    Int1 = 288,
    Ja = 297,
    Jae = 298,
    Jb = 299,
    Jbe = 300,
    Je = 302,
    Jecxz = 303,
    Jg = 304,
    Jge = 305,
    Jl = 306,
    Jle = 307,
    Jmp = 308,
    Jne = 310,
    Jno = 311,
    Jnp = 312,
    Jns = 313,
    Jo = 314,
    Jp = 315,
    Jrcxz = 316,
    Js = 317,
    Lea = 374,
    Mov = 414,
    Movsxd = 451,
    Movups = 453,
    Movzx = 454,
    Mul = 456,
    Neg = 464,
    Nop = 465,
    Not = 466,
    Pop = 590,
    Push = 640,
    Ret = 662,
    Setb = 688,
    Sete = 690,
    Setne = 695,
    Shl = 712,
    Shr = 715,
    Sub = 740,
    Syscall = 746,
    Test = 751,
    Xor = 1518,
    Xorps = 1520,
    Int3 = 1620,
}

impl SupportedMnemonic {
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl TryFrom<Mnemonic> for SupportedMnemonic {
    type Error = AxError;

    fn try_from(mnemonic: Mnemonic) -> Result<Self, Self::Error> {
        Ok(match mnemonic {
            Adc => SupportedMnemonic::Adc,
            Add => SupportedMnemonic::Add,
            And => SupportedMnemonic::And,
            Call => SupportedMnemonic::Call,
            Cdq => SupportedMnemonic::Cdq,
            Cdqe => SupportedMnemonic::Cdqe,
            Cld => SupportedMnemonic::Cld,
            Cmovae => SupportedMnemonic::Cmovae,
            Cmove => SupportedMnemonic::Cmove,
            Cmovne => SupportedMnemonic::Cmovne,
            Cmp => SupportedMnemonic::Cmp,
            Cpuid => SupportedMnemonic::Cpuid,
            Cqo => SupportedMnemonic::Cqo,
            Cwd => SupportedMnemonic::Cwd,
            Dec => SupportedMnemonic::Dec,
            Div => SupportedMnemonic::Div,
            Endbr64 => SupportedMnemonic::Endbr64,
            Idiv => SupportedMnemonic::Idiv,
            Imul => SupportedMnemonic::Imul,
            Inc => SupportedMnemonic::Inc,
            Int => SupportedMnemonic::Int,
            Int1 => SupportedMnemonic::Int1,
            Ja => SupportedMnemonic::Ja,
            Jae => SupportedMnemonic::Jae,
            Jb => SupportedMnemonic::Jb,
            Jbe => SupportedMnemonic::Jbe,
            Je => SupportedMnemonic::Je,
            Jecxz => SupportedMnemonic::Jecxz,
            Jg => SupportedMnemonic::Jg,
            Jge => SupportedMnemonic::Jge,
            Jl => SupportedMnemonic::Jl,
            Jle => SupportedMnemonic::Jle,
            Jmp => SupportedMnemonic::Jmp,
            Jne => SupportedMnemonic::Jne,
            Jno => SupportedMnemonic::Jno,
            Jnp => SupportedMnemonic::Jnp,
            Jns => SupportedMnemonic::Jns,
            Jo => SupportedMnemonic::Jo,
            Jp => SupportedMnemonic::Jp,
            Jrcxz => SupportedMnemonic::Jrcxz,
            Js => SupportedMnemonic::Js,
            Lea => SupportedMnemonic::Lea,
            Mov => SupportedMnemonic::Mov,
            Movsxd => SupportedMnemonic::Movsxd,
            Movups => SupportedMnemonic::Movups,
            Movzx => SupportedMnemonic::Movzx,
            Mul => SupportedMnemonic::Mul,
            Neg => SupportedMnemonic::Neg,
            Nop => SupportedMnemonic::Nop,
            Not => SupportedMnemonic::Not,
            Pop => SupportedMnemonic::Pop,
            Push => SupportedMnemonic::Push,
            Ret => SupportedMnemonic::Ret,
            Setb => SupportedMnemonic::Setb,
            Sete => SupportedMnemonic::Sete,
            Setne => SupportedMnemonic::Setne,
            Shl => SupportedMnemonic::Shl,
            Shr => SupportedMnemonic::Shr,
            Sub => SupportedMnemonic::Sub,
            Syscall => SupportedMnemonic::Syscall,
            Test => SupportedMnemonic::Test,
            Xor => SupportedMnemonic::Xor,
            Xorps => SupportedMnemonic::Xorps,
            Int3 => SupportedMnemonic::Int3,
            _ => {
                fatal_error!(
                    "Conversion from Mnemonic to SupportedMnemonic: mnemonic {:?} is not supported",
                    mnemonic
                );
            }
        })
    }
}
