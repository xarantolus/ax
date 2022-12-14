pub mod add;
pub mod and;
pub mod axecutor;
pub mod call;
pub mod cdq;
pub mod cmovae;
pub mod cmove;
pub mod cmovne;
pub mod cmp;
pub mod cpuid;
pub mod cqo;
pub mod cwd;
mod debug;
pub mod dec;
pub mod div;
pub mod elf;
pub mod endbr64;
pub mod errors;
pub mod execute;
pub mod flags;
pub mod generated;
pub mod hooks;
pub mod idiv;
pub mod imul;
pub mod inc;
pub mod int;
pub mod int1;
pub mod int3;
mod integration_tests;
pub mod ja;
pub mod jae;
pub mod jb;
pub mod jbe;
pub mod je;
pub mod jecxz;
pub mod jg;
pub mod jge;
pub mod jl;
pub mod jle;
pub mod jmp;
pub mod jne;
pub mod jno;
pub mod jnp;
pub mod jns;
pub mod jo;
pub mod jp;
pub mod jrcxz;
pub mod js;
pub mod lea;
mod macros;
pub mod memory;
pub mod mov;
pub mod movsxd;
pub mod movzx;
pub mod mul;
pub mod nop;
pub mod not;
pub mod operand;
pub mod pop;
pub mod push;
pub mod registers;
pub mod ret;
pub mod setb;
pub mod sete;
pub mod setne;
pub mod shl;
pub mod shr;
pub mod sub;
pub mod syscall;
pub mod test;
mod tests;
pub mod xor;
