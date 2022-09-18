#[cfg(test)]
mod test {
    use super::super::axecutor::Axecutor;
    use crate::{
        assert_reg_value, ax_test, instructions::registers::RegisterWrapper, write_reg_value,
    };
    use iced_x86::Register::*;

    // push rax; xor rax, rax; pop rbx
    ax_test![push_rax_pop_rbx; 0x50, 0x48, 0x31, 0xc0, 0x5b;
        |a: &mut Axecutor| {
            write_reg_value!(q; a; RAX; 0x1234567890ABCDEFu64);
            write_reg_value!(q; a; RBX; 0x031591385913u64);

            // Setup stack
            a.reg_write_64(RSP.into(), 0x1000);
            a.mem_init_zero(0x1000-8, 8).unwrap();
        };
        |a: Axecutor| {
            assert_reg_value!(q; a; RAX; 0);
            assert_reg_value!(q; a; RBX; 0x1234567890ABCDEFu64);

            let rsp = a.reg_read_64(RSP.into());
            assert_eq!(rsp, 0x1000);

            assert_eq!(a.mem_read_64(rsp-8).unwrap(), 0x1234567890ABCDEFu64);
        };
        (FLAG_PF | FLAG_ZF; FLAG_CF | FLAG_SF | FLAG_OF)
    ];
}
