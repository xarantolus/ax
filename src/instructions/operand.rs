use iced_x86::Instruction;

use super::{axecutor::Axecutor, errors::AxError, registers::RegisterWrapper};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operand(OperandKind);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandKind {
    Memory {
        base: Option<RegisterWrapper>,
        index: Option<RegisterWrapper>,
        scale: u32,
        displacement: u64,
    },
    Register(RegisterWrapper),
    Immediate {
        data: u64,
        size: i8,
    },
}

impl Axecutor {
    pub(crate) fn mem_addr(&self, o: Operand) -> Result<u64, AxError> {
        match o.0 {
            OperandKind::Memory {
                base,
                index,
                scale,
                displacement,
            } => {
                let mut addr: u64 = 0;
                if let Some(base) = base {
                    addr += self.reg_read_64(base) as u64;
                }
                if let Some(index) = index {
                    addr += self.reg_read_64(index) * (scale as u64);
                }

                // This overflow is explicitly allowed, as x86-64 encodes negative values as signed integers
                // TODO: Does this work correctly on platforms that don't use two's complement? (there's a test for it, so it should fail if this assumption is false)
                addr = addr.wrapping_add(displacement);

                Ok(addr)
            }
            _ => {
                return Err(AxError::from(format!(
                    "Expected memory operand, got {:?}",
                    o.0
                )))
            }
        }
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
                    r => Some(RegisterWrapper::from(r)),
                };
                let index = match i.memory_index() {
                    iced_x86::Register::None => None,
                    r => Some(RegisterWrapper::from(r)),
                };
                let scale = i.memory_index_scale();
                let displacement = i.memory_displacement64();

                Ok(Operand(OperandKind::Memory {
                    base,
                    index,
                    scale,
                    displacement,
                }))
            }
            iced_x86::OpKind::Register => Ok(Operand(OperandKind::Register(
                RegisterWrapper::from(i.op_register(operand_idx)),
            ))),
            iced_x86::OpKind::Immediate8 => Ok(Operand(OperandKind::Immediate {
                data: i.immediate8() as u64,
                size: 1,
            })),
            iced_x86::OpKind::Immediate8_2nd => Ok(Operand(OperandKind::Immediate {
                data: i.immediate8_2nd() as u64,
                size: 1,
            })),
            iced_x86::OpKind::Immediate16 => Ok(Operand(OperandKind::Immediate {
                data: i.immediate16() as u64,
                size: 2,
            })),
            iced_x86::OpKind::Immediate32 => Ok(Operand(OperandKind::Immediate {
                data: i.immediate32() as u64,
                size: 4,
            })),
            iced_x86::OpKind::Immediate64 => Ok(Operand(OperandKind::Immediate {
                data: i.immediate64(),
                size: 8,
            })),
            iced_x86::OpKind::Immediate8to16 => Ok(Operand(OperandKind::Immediate {
                data: i.immediate8to16() as u64,
                size: 2,
            })),
            iced_x86::OpKind::Immediate8to32 => Ok(Operand(OperandKind::Immediate {
                data: i.immediate8to32() as u64,
                size: 4,
            })),
            iced_x86::OpKind::Immediate8to64 => Ok(Operand(OperandKind::Immediate {
                data: i.immediate8to64() as u64,
                size: 8,
            })),
            iced_x86::OpKind::Immediate32to64 => Ok(Operand(OperandKind::Immediate {
                data: i.immediate32to64() as u64,
                size: 8,
            })),
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
    use iced_x86::Register;

    use super::{Axecutor, Operand, OperandKind::*};

    const TEST_RIP_VALUE: u64 = 0x1000;

    macro_rules! operand_test {
		[$test_name:ident; $($bytes:expr),*; $expected:expr] => {
			#[test]
			fn $test_name () {
				let expected : Vec<Operand> = $expected;
				let axecutor = Axecutor::new(&[$($bytes),*], TEST_RIP_VALUE).expect("Failed to create axecutor");
				assert_eq!(axecutor.instructions.len(), 1);
				let instruction = axecutor.instructions[0];

				assert_eq!(instruction.op_count(), expected.len() as u32);
				for i in 0..expected.len() {
					let operand = axecutor.instruction_operand(instruction, i as u32).expect("Failed to get operand");

					assert_eq!(operand, expected[i]);
				}
			}
		};
		[$test_name:ident; $($bytes:expr),*; $expected:expr; $setup:expr; $memaddrs:expr] => {
			#[test]
			fn $test_name () {
				let expected : Vec<Operand> = $expected;
				let mut axecutor = Axecutor::new(&[$($bytes),*], 0x1000).expect("Failed to create axecutor");
				assert_eq!(axecutor.instructions.len(), 1);
				let instruction = axecutor.instructions[0];

				let mut mem_addr_counter: usize = 0;

				assert_eq!(instruction.op_count(), expected.len() as u32);

				$setup(&mut axecutor);

				for i in 0..expected.len() {
					let operand = axecutor.instruction_operand(instruction, i as u32).expect("Failed to get operand");
					if let Memory { .. } = operand.0 {
						let mem_addr = axecutor.mem_addr(operand).expect("Failed to get memory address");
						assert_eq!(mem_addr, $memaddrs[mem_addr_counter]);
						mem_addr_counter += 1;
					}

					assert_eq!(operand, expected[i]);
				}

				assert_eq!(mem_addr_counter, $memaddrs.len());
			}
		};
	}

    // mov byte ptr [0], 1
    operand_test![mov_byte_ptr_0_1;
        0xc6, 0x4, 0x25, 0x0, 0x0, 0x0, 0x0, 0x1;
        vec![
            Operand(Memory {
                base: None,
                index: None,
                scale: 1,
                displacement: 0,
            }),
            Operand(Immediate { data: 1, size: 1 }),
        ]
    ];

    // mov byte ptr [rsp], 1
    operand_test![mov_byte_ptr_rsp_1;
        0xc6, 0x4, 0x24, 0x1;
        vec![
            Operand(Memory {
                base: Some(Register::RSP.into()),
                index: None,
                scale: 1,
                displacement: 0,
            }),
            Operand(Immediate { data: 1, size: 1 }),
        ];
        |a: &mut Axecutor| {
            use iced_x86::Register::*;
            a.reg_write_64(RSP.into(), 0x1000)
        };
        vec![
            0x1000
        ]
    ];

    // mov dword ptr [rsp], 1
    operand_test![mov_dword_ptr_rsp_1;
        0xc7, 0x4, 0x24, 0x1, 0x0, 0x0, 0x0;
        vec![
            Operand(Memory {
                base: Some(Register::RSP.into()),
                index: None,
                scale: 1,
                displacement: 0,
            }),
            Operand(Immediate { data: 1, size: 4 }),
        ];
        |a: &mut Axecutor| {
            use iced_x86::Register::*;
            a.reg_write_64(RSP.into(), 0x1000)
        };
        vec![
            0x1000
        ]
    ];

    // mov [rsp+1], r15d
    operand_test![mov_rsp1_r15d;
        0x44, 0x89, 0x7c, 0x24, 0x1;
        vec![
            Operand(Memory {
                base: Some(Register::RSP.into()),
                index: None,
                scale: 1,
                displacement: 1,
            }),
            Operand(Register(Register::R15D.into())),
        ];
        |a: &mut Axecutor| {
            use iced_x86::Register::*;
            a.reg_write_64(RSP.into(), 0x1000)
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
            Operand(Memory {
                base: Some(Register::RSP.into()),
                index: None,
                scale: 1,
                displacement: u64::MAX,
            }),
            Operand(Register(Register::R15D.into())),
        ];
        |a: &mut Axecutor| {
            use iced_x86::Register::*;
            a.reg_write_64(RSP.into(), 0x1000)
        };
        vec![
            0x0fff
        ]
    ];

    // xor qword ptr [r11+4*rcx], 1
    operand_test![xor_qword_ptr_r11_4_rcx_1;
        0x49, 0x83, 0x34, 0x8b, 0x1;
        vec![
            Operand(Memory {
                base: Some(Register::R11.into()),
                index: Some(Register::RCX.into()),
                scale: 4,
                displacement: 0,
            }),
            Operand(Immediate { data: 1, size: 8 }),
        ];
        |a: &mut Axecutor| {
            use iced_x86::Register::*;
            a.reg_write_64(R11.into(), 0x8001);
            a.reg_write_64(RCX.into(), 5);
        };
        vec![
            0x8015
        ]
    ];

    // xor [rip+0x5], rbx
    operand_test![rip_relative_constant;
        0x48, 0x31, 0x1d, 0x5, 0x0, 0x0, 0x0;
        vec![
            Operand(Memory {
                base: None, // RIP is ignored
                index: None,
                scale: 1,
                // RIP + Instruction size + Displacement
                displacement: TEST_RIP_VALUE + 0x7 + 0x5,
            }),
            Operand(Register(Register::RBX.into())),
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
            Operand(Memory {
                base: None,
                index: None,
                scale: 1,
                displacement: TEST_RIP_VALUE + 0x7 - 0x20,
            }),
            Operand(Immediate { data: 5, size: 1 }),
        ];
        |_: &mut Axecutor| { };
        vec![
            TEST_RIP_VALUE + 0x7 - 0x20
        ]
    ];
}
