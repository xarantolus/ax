#![cfg(not(any(test, target_arch = "wasm32")))]

use std::println;

use ax_x86::{
    auto::generated::SupportedMnemonic::Syscall,
    axecutor::Axecutor,
    helpers::errors::AxError,
    state::{hooks::HookResult, registers::SupportedRegister},
};

fn main() {
    async_std::task::block_on(async {
        match main_impl().await {
            Ok(exit_code) => {
                std::process::exit(exit_code);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    });
}

async fn main_impl() -> Result<i32, AxError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let envp: Vec<String> = std::env::vars().map(|(k, v)| format!("{k}={v}")).collect();

    let (elf_path, argv) = args
        .split_first()
        .ok_or_else(|| AxError::from("No arguments provided"))?;

    let binary = std::fs::read(elf_path)
        .map_err(|e| AxError::from(format!("Failed to read file {elf_path}: {e}")))?;

    println!(
        "Emulating {} with ax v{}, {}",
        elf_path,
        ax_x86::version_info::version(),
        ax_x86::version_info::commit()
    );

    let mut ax = Axecutor::from_binary(binary.as_slice())?;

    ax.init_stack_program_start(0x2000, Vec::from(argv), envp)?;

    ax.hook_before_mnemonic_native(Syscall, &|ax: &mut Axecutor, _| {
        let syscall_num = ax.reg_read_64(SupportedRegister::RAX)?;
        let rdi = ax.reg_read_64(SupportedRegister::RDI)?;
        let rsi = ax.reg_read_64(SupportedRegister::RSI)?;
        let rdx = ax.reg_read_64(SupportedRegister::RDX)?;

        match syscall_num {
            // Write
            1 => {
                // rdi must be 0-2 (stdin, stdout, stderr) -- yes, we allow writing to stdin
                if rdi > 2 {
                    return Err(AxError::from("write: invalid file descriptor").into());
                }

                let result_buf = ax.mem_read_bytes(rsi, rdx).map_err(|e| {
                    AxError::from(format!("write: failed to read memory at {rsi}: {e}"))
                })?;
                let output_text = String::from_utf8(result_buf).map_err(|e| {
                    AxError::from(format!(
                        "write: failed to convert memory at {rsi} to utf8: {e}"
                    ))
                })?;

                if rdi == 2 {
                    eprint!("{output_text}");
                } else {
                    print!("{output_text}");
                }

                // Return number of bytes written
                ax.reg_write_64(SupportedRegister::RAX, rdx).map_err(|e| {
                    AxError::from(format!("write: failed to write return value to RAX: {e}"))
                })?;
            }
            // Exit
            60 => {
                ax.stop();
            }
            _ => {
                return Err(AxError::from(format!("Unsupported syscall: {syscall_num}")).into());
            }
        }

        Ok(HookResult::Handled)
    })?;

    // add axecutor string to error message
    ax.execute()
        .await
        .map_err(|e| AxError::from(format!("{}\nAxecutor state:\n{}", e, ax.to_string())))?;

    let exit_code = ax.reg_read_64(SupportedRegister::RAX)?;

    Ok(exit_code as i32)
}
