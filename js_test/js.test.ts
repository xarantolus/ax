import { beforeAll, it, describe, expect, beforeEach } from 'vitest';
import { readFile } from 'fs/promises';

const { version, commit, Axecutor, Register, Mnemonic } = require('ax-x86');

let correctX86Code = new Uint8Array([0x48, 0x89, 0xd8]); // mov rax, rbx

describe('Axecutor', () => {
	it('should execute correct code', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);

		axecutor.reg_write_64(Register.RBX, 0x1234567890abcdefn);
		await axecutor.execute();

		expect(axecutor.reg_read_64(Register.RAX)).toBe(0x1234567890abcdefn);
	});
});

describe('Use BigInt for all register interactions', () => {
	let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);

	it('should use BigInt for 8-bit registers', () => {
		expect(axecutor.reg_read_8(Register.AL)).toBeTypeOf('bigint');
		expect(axecutor.reg_write_8(Register.AH, 0x12n));
		expect(() => axecutor.reg_write_8(Register.AH, (0x12 as any))).toThrow();
	});

	it('should use BigInt for 16-bit registers', () => {
		expect(axecutor.reg_read_16(Register.AX)).toBeTypeOf('bigint');
		expect(axecutor.reg_write_16(Register.AX, 0x1234n));
		expect(() => axecutor.reg_write_16(Register.AX, (0x1234 as any))).toThrow();
	});

	it('should use BigInt for 32-bit registers', () => {
		expect(axecutor.reg_read_32(Register.EAX)).toBeTypeOf('bigint');
		expect(axecutor.reg_write_32(Register.EAX, 0x12345678n));
		expect(() => axecutor.reg_write_32(Register.EAX, (0x12345678 as any))).toThrow();
	});

	it('should use BigInt for 64-bit registers', () => {
		expect(axecutor.reg_read_64(Register.RAX)).toBeTypeOf('bigint');
		expect(axecutor.reg_write_64(Register.RAX, 0x1234567890abcdefn));
		expect(() => axecutor.reg_write_64(Register.RAX, (0x1234567890abcdef as any))).toThrow();
	});
});

describe('Use BigInt for all memory interactions', () => {
	let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
	let mem_start = 0x5000n;

	beforeAll(() => {
		axecutor.mem_init_zero(mem_start, 8n);
	});

	it('should use BigInt for 8-bit memory', () => {
		expect(axecutor.mem_read_8(mem_start)).toBeTypeOf('bigint');
		expect(axecutor.mem_write_8(mem_start, 0x12n));
		expect(() => axecutor.mem_write_8(mem_start, (0x12 as any))).toThrow();
	});

	it('should use BigInt for 16-bit memory', () => {
		expect(axecutor.mem_read_16(mem_start)).toBeTypeOf('bigint');
		expect(axecutor.mem_write_16(mem_start, 0x1234n));
		expect(() => axecutor.mem_write_16(mem_start, (0x1234 as any))).toThrow();
	});

	it('should use BigInt for 32-bit memory', () => {
		expect(axecutor.mem_read_32(mem_start)).toBeTypeOf('bigint');
		expect(axecutor.mem_write_32(mem_start, 0x12345678n));
		expect(() => axecutor.mem_write_32(mem_start, (0x12345678 as any))).toThrow();
	});

	it('should use BigInt for 64-bit memory', () => {
		expect(axecutor.mem_read_64(mem_start)).toBeTypeOf('bigint');
		expect(axecutor.mem_write_64(mem_start, 0x1234567890abcdefn));
		expect(() => axecutor.mem_write_64(mem_start, (0x1234567890abcdef as any))).toThrow();
	});
});


describe('Run ELF binaries', () => {
	let output = "";

	async function syscallHandler(ax) {
		let syscall_num = ax.reg_read_64(Register.RAX);
		let rdi = ax.reg_read_64(Register.RDI);
		let rsi = ax.reg_read_64(Register.RSI);
		let rdx = ax.reg_read_64(Register.RDX);

		switch (syscall_num) {
			case 1n: {
				// WRITE syscall MUST write to stdout or stderr (stdin supported for compatibility)
				if (rdi != 0n && rdi != 1n && rdi != 2n) {
					throw new Error(`WRITE syscall: cannot write non-std{out,err} (!= 1,2) fds, but tried ${rdi}`);
				}
				// Read data we should write from memory
				let result_buf = ax.mem_read_bytes(rsi, rdx);

				// Decode to string
				let result_str = new TextDecoder().decode(result_buf);

				// Do something with the string
				output += result_str;

				ax.reg_write_64(Register.RAX, rdx);

				return ax.commit();
			}
			case 60n: {
				// EXIT syscall
				return ax.stop();
			}
		}

		throw `Unhandled syscall ${syscall_num}`;
	}

	beforeEach(() => {
		output = "";
	});

	it('should run a simple hello world program', async () => {
		let elf = await readFile(
			new URL('../testdata/hello_world.bin', import.meta.url)
		);

		let axecutor = Axecutor.from_binary(elf);

		axecutor.hook_before_mnemonic(Mnemonic.Syscall, syscallHandler)

		await axecutor.execute();

		expect(output).toBe("Hello, World!\n");
	});

	it('should run a simple alphabet program', async () => {
		let elf = await readFile(
			new URL('../testdata/alphabet.bin', import.meta.url)
		);

		let axecutor = Axecutor.from_binary(elf);

		axecutor.hook_before_mnemonic(Mnemonic.Syscall, syscallHandler)

		await axecutor.execute();

		expect(output).toBe("abcdefghijklmnopqrstuvwxyz\n");
	});

	it('should run a more complex argument print program', async () => {
		let elf = await readFile(
			new URL('../testdata/args.bin', import.meta.url)
		);

		let axecutor = Axecutor.from_binary(elf);
		axecutor.init_stack_program_start(
			1024n, // Stack size
			["/bin/my_binary", "arg1", "arg2"], // argv
			["COLORTERM=truecolor", "TERM=xterm-256color"] // environment variables
		);
		axecutor.hook_before_mnemonic(Mnemonic.Syscall, syscallHandler);

		await axecutor.execute();

		expect(output).toBe(`--------------------------------------------------
argv values:
--------------------------------------------------
/bin/my_binary
arg1
arg2
--------------------------------------------------
envp values:
--------------------------------------------------
COLORTERM=truecolor
TERM=xterm-256color
`);
	});
});

describe('Basic information is exposed', () => {
	it('should expose a version', () => {
		let v = version();
		expect(v).toBeTypeOf('string')
		expect(v.split('.').length).toBe(3);
	});

	it('should expose a commit hash', () => {
		let c = commit();
		expect(c).toBeTypeOf('string');
		expect(c.length).toBe(40);
		// hex chars only
		expect(c).toMatch(/^[0-9a-f]+$/);
	});
});

describe('Segment registers work', () => {
	it('should read and write segment registers', () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
		axecutor.write_fs(0x1234n);
		expect(axecutor.read_fs()).toBe(0x1234n);
	});

	it('should read and write segment registers ion hooks', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);

		axecutor.write_fs(0n);

		axecutor.hook_before_mnemonic(Mnemonic.Mov, (ax) => {
			ax.write_fs(0x1234n);

			return ax.commit();
		});

		await axecutor.execute();

		expect(axecutor.read_fs()).toBe(0x1234n);
	});
});

describe('Mnemonic hooks', () => {
	it('should execute a hook before a mnemonic', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
		let hook_called = false;
		axecutor.hook_before_mnemonic(Mnemonic.Mov, (ax) => {
			hook_called = true;

			return ax.commit();
		});
		await axecutor.execute();
		expect(hook_called).toBe(true);
	});

	it('should execute a hook after a mnemonic', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
		let hook_called = false;
		axecutor.hook_after_mnemonic(Mnemonic.Mov, (ax) => {
			hook_called = true;

			return ax.commit();
		});
		await axecutor.execute();
		expect(hook_called).toBe(true);
	});

	it('should not keep changes when returning unchanged() from before hook', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
		// RCX is not used in the code
		axecutor.reg_write_64(Register.RCX, 0n);
		axecutor.hook_before_mnemonic(Mnemonic.Mov, (ax) => {
			ax.reg_write_64(Register.RCX, 0x1234n);

			return ax.unchanged();
		});
		await axecutor.execute();
		expect(axecutor.reg_read_64(Register.RCX)).toBe(0n);
	});

	it('should keep changes when returning commit() from before hook', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
		// RCX is not used in the code
		axecutor.reg_write_64(Register.RCX, 0n);
		axecutor.hook_before_mnemonic(Mnemonic.Mov, (ax) => {
			ax.reg_write_64(Register.RCX, 0x1234n);

			return ax.commit();
		});
		await axecutor.execute();
		expect(axecutor.reg_read_64(Register.RCX)).toBe(0x1234n);
	});

	it('should allow promises as hook functions', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
		let hook_called = false;
		axecutor.hook_after_mnemonic(Mnemonic.Mov, async (ax) => {
			hook_called = true;

			return ax.commit();
		});
		await axecutor.execute();
		expect(hook_called).toBe(true);
	});


	it('should error when nothing is returned from a hook', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
		axecutor.hook_after_mnemonic(Mnemonic.Mov, () => {
			// no return
		});
		await expect(axecutor.execute()).rejects.toThrow();
	});

	it('should error when an invalid value is returned from a hook', async () => {
		let axecutor = new Axecutor(correctX86Code, 0x1000n, 0x1000n);
		axecutor.hook_after_mnemonic(Mnemonic.Mov, () => {
			return 123;
		});
		await expect(axecutor.execute()).rejects.toThrow();
	});
});
