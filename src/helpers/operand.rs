use std::convert::TryFrom;

use iced_x86::Instruction;

use crate::helpers::debug::debug_log;

use crate::{axecutor::Axecutor, helpers::errors::AxError, state::registers::SupportedRegister};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemOperand {
    base: Option<SupportedRegister>,
    index: Option<SupportedRegister>,
    segment: Option<SupportedSegmentRegister>,
    scale: u32,
    displacement: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SupportedSegmentRegister {
    DS,
    ES,
    SS,
    FS,
    GS,
}

impl TryFrom<iced_x86::Register> for SupportedSegmentRegister {
    type Error = AxError;

    fn try_from(value: iced_x86::Register) -> Result<Self, Self::Error> {
        match value {
            iced_x86::Register::DS => Ok(SupportedSegmentRegister::DS),
            iced_x86::Register::ES => Ok(SupportedSegmentRegister::ES),
            iced_x86::Register::SS => Ok(SupportedSegmentRegister::SS),
            iced_x86::Register::FS => Ok(SupportedSegmentRegister::FS),
            iced_x86::Register::GS => Ok(SupportedSegmentRegister::GS),
            _ => Err(AxError::from(format!(
                "Unsupported segment register: {value:?}"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Memory(MemOperand),
    Register(SupportedRegister),
    Immediate { data: u64, size: i8 },
}

impl TryFrom<Operand> for SupportedRegister {
    type Error = AxError;

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Register(reg) => Ok(reg),
            _ => Err(AxError::from(format!(
                "Cannot convert operand to register: {value:?}"
            ))),
        }
    }
}

impl TryFrom<Operand> for u8 {
    type Error = AxError;

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Immediate { data, size } => {
                debug_assert_eq!(
                    size, 1,
                    "Expected immediate operand of size 1, got size {size}"
                );
                Ok(data as u8)
            }
            _ => Err(AxError::from(format!(
                "Cannot convert operand to u8: {value:?}"
            ))),
        }
    }
}

impl TryFrom<Operand> for u16 {
    type Error = AxError;

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Immediate { data, size } => {
                debug_assert_eq!(
                    size, 2,
                    "Expected immediate operand of size 2, got size {size}"
                );
                Ok(data as u16)
            }
            _ => Err(AxError::from(format!(
                "Cannot convert operand to u16: {value:?}"
            ))),
        }
    }
}

impl TryFrom<Operand> for u32 {
    type Error = AxError;

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Immediate { data, size } => {
                debug_assert_eq!(
                    size, 4,
                    "Expected immediate operand of size 4, got size {size}"
                );
                Ok(data as u32)
            }
            _ => Err(AxError::from(format!(
                "Cannot convert operand to u32: {value:?}"
            ))),
        }
    }
}

impl TryFrom<Operand> for u64 {
    type Error = AxError;

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Immediate { data, size } => {
                debug_assert_eq!(
                    size, 8,
                    "Expected immediate operand of size 8, got size {size}"
                );
                Ok(data)
            }
            _ => Err(AxError::from(format!(
                "Cannot convert operand to u64: {value:?}"
            ))),
        }
    }
}

impl Axecutor {
    pub(crate) fn mem_addr(&self, o: MemOperand) -> u64 {
        let MemOperand {
            base,
            index,
            scale,
            displacement,
            segment,
        } = o;
        let mut addr: u64 = 0;
        if let Some(base) = base {
            addr = addr.wrapping_add(
                self.reg_read_64(base)
                    .expect("reading memory operand base register"),
            );
        }
        if let Some(index) = index {
            addr = addr.wrapping_add(
                self.reg_read_64(index)
                    .expect("reading memory operand index register")
                    .wrapping_mul(scale as u64),
            );
        }

        // This overflow is explicitly allowed, as x86-64 encodes negative values as signed integers
        addr = addr.wrapping_add(displacement);

        if let Some(reg) = segment {
            match reg {
                SupportedSegmentRegister::FS => {
                    debug_log!(
                        "Adding FS segment offset {:#x} to memory address: {:#x}",
                        self.state.fs,
                        addr
                    );
                    addr = addr.wrapping_add(self.state.fs);
                }
                SupportedSegmentRegister::GS => {
                    debug_log!(
                        "Adding GS segment offset {:#x} to memory address: {:#x}",
                        self.state.gs,
                        addr
                    );
                    addr = addr.wrapping_add(self.state.gs);
                }
                _ => {
                    // Others are always zero
                    // AMD Architecture Programmer's Manual, 2.1.2 Segment Registers:
                    // "For references to the DS, ES, or SS segments in 64-bit mode, the processor assumes that the base for each of these segments is zero"
                }
            }
        }

        addr
    }

    pub(crate) fn instruction_operands_2(
        &self,
        i: Instruction,
    ) -> Result<(Operand, Operand), AxError> {
        let dest = self.instruction_operand(i, 0)?;
        let src = self.instruction_operand(i, 1)?;

        Ok((dest, src))
    }

    pub(crate) fn instruction_operand(
        &self,
        i: Instruction,
        operand_idx: u32,
    ) -> Result<Operand, AxError> {
        assert!(
            operand_idx < i.op_count(),
            "Operand index {} out of bounds on instruction with {} operands",
            operand_idx,
            i.op_count()
        );

        match i.op_kind(operand_idx) {
            iced_x86::OpKind::Memory => {
                let base = match i.memory_base() {
                    iced_x86::Register::None => None,
                    // If base is RIP, we can use the displacement as-it. No need to add it to the memory address
                    iced_x86::Register::RIP => None,
                    r => Some(SupportedRegister::try_from(r)?),
                };
                let index = match i.memory_index() {
                    iced_x86::Register::None => None,
                    r => Some(SupportedRegister::try_from(r)?),
                };
                let scale = i.memory_index_scale();
                let displacement = i.memory_displacement64();

                let segment = if i.memory_segment() == iced_x86::Register::None {
                    None
                } else {
                    Some(SupportedSegmentRegister::try_from(i.memory_segment())?)
                };

                Ok(Operand::Memory(MemOperand {
                    base,
                    index,
                    scale,
                    displacement,
                    segment,
                }))
            }
            iced_x86::OpKind::Register => Ok(Operand::Register(SupportedRegister::try_from(
                i.op_register(operand_idx),
            )?)),
            iced_x86::OpKind::Immediate8 => Ok(Operand::Immediate {
                data: i.immediate8() as u64,
                size: 1,
            }),
            iced_x86::OpKind::Immediate8_2nd => Ok(Operand::Immediate {
                data: i.immediate8_2nd() as u64,
                size: 1,
            }),
            iced_x86::OpKind::Immediate16 => Ok(Operand::Immediate {
                data: i.immediate16() as u64,
                size: 2,
            }),
            iced_x86::OpKind::Immediate32 => Ok(Operand::Immediate {
                data: i.immediate32() as u64,
                size: 4,
            }),
            iced_x86::OpKind::Immediate64 => Ok(Operand::Immediate {
                data: i.immediate64(),
                size: 8,
            }),
            iced_x86::OpKind::Immediate8to16 => Ok(Operand::Immediate {
                data: i.immediate8to16() as u64,
                size: 2,
            }),
            iced_x86::OpKind::Immediate8to32 => Ok(Operand::Immediate {
                data: i.immediate8to32() as u64,
                size: 4,
            }),
            iced_x86::OpKind::Immediate8to64 => Ok(Operand::Immediate {
                data: i.immediate8to64() as u64,
                size: 8,
            }),
            iced_x86::OpKind::Immediate32to64 => Ok(Operand::Immediate {
                data: i.immediate32to64() as u64,
                size: 8,
            }),
            _ => Err(AxError::from(format!(
                "instruction_operand {}: unimplemented operand kind {:?}",
                operand_idx,
                i.op_kind(operand_idx)
            ))),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::helpers::operand::{MemOperand, SupportedSegmentRegister};
    use crate::helpers::tests::{assert_reg_value, ax_test, write_reg_value};
    use crate::state::registers::SupportedRegister;
    use iced_x86::Register::*;

    use super::{Axecutor, Operand, Operand::*};

    const TEST_RIP_VALUE: u64 = 0x1000;

    macro_rules! operand_test {
        [$test_name:ident; $($bytes:expr),*; $expected:expr] => {
			crate::helpers::tests::test_async![$test_name; async {
				let expected : Vec<Operand> = $expected;
				let axecutor = Axecutor::new(&[$($bytes),*], TEST_RIP_VALUE, TEST_RIP_VALUE).expect("Failed to create axecutor");

                let instruction = axecutor.decode_next().expect("Failed to get instruction");

				assert_eq!(instruction.op_count(), expected.len() as u32, "Expected {} operands, got {}", expected.len(), instruction.op_count());
				for i in 0..expected.len() {
					let operand = axecutor.instruction_operand(instruction, i as u32).expect("Failed to get operand");

					assert_eq!(operand, expected[i], "Operand {} mismatch", i);
				}
			}];
		};
		[$test_name:ident; $($bytes:expr),*; $expected:expr; $setup:expr; $memaddrs:expr] => {
			crate::helpers::tests::test_async![$test_name; async {
				let expected : Vec<Operand> = $expected;
				let mut axecutor = Axecutor::new(&[$($bytes),*], 0x1000, 0x1000).expect("Failed to create axecutor");
				let instruction = axecutor.decode_next().expect("Failed to get instruction");

				let mut mem_addr_counter: usize = 0;

				assert_eq!(instruction.op_count(), expected.len() as u32, "Expected {} operands, got {}", expected.len(), instruction.op_count());

				$setup(&mut axecutor);

				for i in 0..expected.len() {
					let operand = axecutor.instruction_operand(instruction, i as u32).expect("Failed to get operand");
                    assert_eq!(operand, expected[i], "Operand {} does not match", i);
					if let Memory(m) = operand {
						let mem_addr = axecutor.mem_addr(m);
						assert_eq!(mem_addr, $memaddrs[mem_addr_counter], "Memory address mismatch for operand {:?}", m);
						mem_addr_counter += 1;
					}

				}

				assert_eq!(mem_addr_counter, $memaddrs.len(), "Provided memory addresses do not match the number of memory operands");
			}];
		};
	}

    // mov byte ptr [0], 1
    operand_test![mov_byte_ptr_0_1;
        0xc6, 0x4, 0x25, 0x0, 0x0, 0x0, 0x0, 0x1;
        vec![
            Memory (MemOperand {
                base: Option::None,
                index: Option::None,
                scale: 1,
                displacement: 0,
                segment: Some(SupportedSegmentRegister::DS),
            }),
            Immediate { data: 1, size: 1 },
        ]
    ];

    // mov byte ptr [rsp], 1
    operand_test![mov_byte_ptr_rsp_1;
        0xc6, 0x4, 0x24, 0x1;
        vec![
            (Memory (MemOperand{
                base: Some(SupportedRegister::RSP),
                index: Option::None,
                scale: 1,
                displacement: 0,
                segment: Some(SupportedSegmentRegister::SS),
            })),
            Immediate { data: 1, size: 1 },
        ];
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x1000);
        };
        vec![
            0x1000
        ]
    ];

    // mov dword ptr [rsp], 1
    operand_test![mov_dword_ptr_rsp_1;
        0xc7, 0x4, 0x24, 0x1, 0x0, 0x0, 0x0;
        vec![
            Memory (MemOperand{
                base: Some(SupportedRegister::RSP),
                index: Option::None,
                scale: 1,
                displacement: 0,
                segment: Some(SupportedSegmentRegister::SS),
            }),
            Immediate { data: 1, size: 4 },
        ];
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x1000);
        };
        vec![
            0x1000
        ]
    ];

    // mov [rsp+1], r15d
    operand_test![mov_rsp1_r15d;
        0x44, 0x89, 0x7c, 0x24, 0x1;
        vec![
            Memory(MemOperand {
                base: Some(SupportedRegister::RSP),
                index: Option::None,
                scale: 1,
                displacement: 1,
                segment: Some(SupportedSegmentRegister::SS),
            }),
            Register(SupportedRegister::R15D),
        ];
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x1000);
        };
        vec![
            0x1001
        ]
    ];

    // mov [rsp-1], r15d
    // This test also tests if the negative displacement works on platforms that don't use two's complement
    operand_test![twos_complement_wraparound_negative_displacement;
        0x44, 0x89, 0x7c, 0x24, 0xff;
        vec![
            Memory(MemOperand{
                base: Some(SupportedRegister::RSP),
                index: Option::None,
                scale: 1,
                displacement: u64::MAX,
                segment: Some(SupportedSegmentRegister::SS),
            }),
            Register(SupportedRegister::R15D),
        ];
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RSP; 0x1000);
        };
        vec![
            0x0fff
        ]
    ];

    // xor qword ptr [r11+4*rcx], 1
    operand_test![xor_qword_ptr_r11_4_rcx_1;
        0x49, 0x83, 0x34, 0x8b, 0x1;
        vec![
            Memory (MemOperand{
                base: Some(SupportedRegister::R11),
                index: Some(SupportedRegister::RCX),
                scale: 4,
                displacement: 0,
                segment: Some(SupportedSegmentRegister::DS),
            }),
            Immediate { data: 1, size: 8 },
        ];
        |a: &mut Axecutor| {
            write_reg_value!(q; a; R11; 0x8001);
            write_reg_value!(q; a; RCX; 5);
        };
        vec![
            0x8015
        ]
    ];

    // xor [rip+0x5], rbx
    operand_test![rip_relative_constant;
        0x48, 0x31, 0x1d, 0x5, 0x0, 0x0, 0x0;
        vec![
            Memory(MemOperand {
                base: Option::None, // RIP is ignored
                index: Option::None,
                scale: 1,
                // RIP + Instruction size + Displacement
                displacement: TEST_RIP_VALUE + 0x7 + 0x5,
                segment: Some(SupportedSegmentRegister::DS),
            }),
            Register(SupportedRegister::RBX),
        ];
        |_: &mut Axecutor| {
            // RIP has a default value defined above
        };
        vec![
            TEST_RIP_VALUE + 0x7 + 0x5
        ]
    ];

    // xor byte ptr [rip-0x20], 5
    operand_test![xor_byte_ptr_rip0x20_5;
        0x80, 0x35, 0xe0, 0xff, 0xff, 0xff, 0x5;
        vec![
            Memory(MemOperand{
                base: Option::None,
                index: Option::None,
                scale: 1,
                displacement: TEST_RIP_VALUE + 0x7 - 0x20,
                segment: Some(SupportedSegmentRegister::DS),
            }),
            Immediate { data: 5, size: 1 },
        ];
        |_: &mut Axecutor| { };
        vec![
            TEST_RIP_VALUE + 0x7 - 0x20
        ]
    ];

    // lea rcx, [rbp+rax-1]
    ax_test![lea_rcx_rbp_rax_1; 0x48, 0x8d, 0x4c, 0x5, 0xff;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RBP; 0x8);
            write_reg_value!(q; a; RAX; 0xffffffffffffffffu64);
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RCX; 0x6);
            assert_reg_value!(q; a; RBP; 0x8);
            assert_reg_value!(q; a; RAX; 0xffffffffffffffffu64);
        };
        (0; FLAG_CF | FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
    ];
}
