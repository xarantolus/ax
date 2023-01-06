# [ax](https://ax.010.one)
This is a minimal x86-64 emulator for WebAssembly. It executes real machine code and can be used to emulate x86-64 user-space programs in the browser.

Note that not all implemented instructions work exactly the same way as on real hardware, but the goal is to be as close as possible while staying reasonable. Currently only around <!-- stats-count-marker -->58 mnemonics/275 opcodes<!-- stats-count-marker --> are implemented. This count can also be found via the [`stats.py`](stats.py) script.

Additionally, this repository contains scripts that should be interesting for anyone who wants to write an x86-64 emulator. The most important one, [`t.py`](t.py), automatically generates test cases for an instruction by trying out different inputs and thus finding many flag combinations. See [automatically generate test cases](#automatically-generate-test-cases) for more information.

## Try it out!
You can try out the emulator right now by visiting [the website](https://ax.010.one), selecting a suitable ELF binary and clicking "Run". The emulator will then execute the binary and show the output. Note that currently support for ELF binaries is limited/buggy (there are some problems getting libc to work), you can however use binaries from the [`examples/programs`](examples/programs) directory. The source code for this site is in the [`examples/web`](examples/web) directory.

Other than that, you can also find it in use on the [MemeAssembly Playground](https://memeasm.010.one). MemeAssembly is a meme programming language that compiles to x86-64 instructions, which are executed by this emulator. The site also emulates some syscalls like `read`, `write` and `exit` to make the programs work.

## How to use
The emulator is compiled to WebAssembly and can be used as a JavaScript Module. This works in modern browsers.

The recommended approach is to just install the [NPM module](https://www.npmjs.com/package/ax-x86):

```sh
npm i ax-x86
```

Before using any functions, you have to make sure the WASM binary has been downloaded using the default `init` function:

```js
import init, { version } from 'ax-x86';
// This will download the WASM binary and initialize the module
await init();
// Now you can use the module
console.log("ax version:", version());
```

Two warnings/pitfalls when using this emulator:
* Make sure that all numbers are passed as `bigint`, which can be done using an `n` suffix. `0x1000n` is a `bigint` literal (which is what we want), `0x1000` is a `number` (which will *not* work)
* When using frontend frameworks, it is recommended to await the `init` function before your components are mounted, e.g. in a `setup` function. This will make sure the WASM binary is downloaded before the component is rendered. You can look at the [this Vue component](examples/web/src/components/Demo.vue) for an example.


### Simple emulation of instruction
The following is a simple example that executes a few instructions and prints the calculated result.

<details>
<summary>Open for more info on the example</summary>

```js
import init, { Axecutor, Mnemonic, Register, version } from 'ax-x86';
await init();

// Define bytes for x86 instructions:
let code = new Uint8Array([
    // mov rax, 0xff
    0x48, 0xc7, 0xc0, 0xff, 0, 0, 0,
    // mov rbx, 0xf
    0x48, 0xc7, 0xc3, 0xf, 0, 0, 0,
    // and rax, rbx
    0x48, 0x21, 0xd8
]);

// Create a new emulator instance
// You could also create an instance from an ELF/Linux binary using `Axecutor.from_binary` instead
let ax = new Axecutor(
    code,
    // Code start address, this is where the first instruction byte is located
    0x1000n,
    // Entrypoint address, this is where execution starts. It is usually, but not always, the same as the code start address
    0x1000n
);
console.log("Initial state:", ax.toString());

// One could set up a stack of size 0x1000 here, but it's not necessary for this example
// This automatically writes the stack pointer to RSP
// let stack_addr = ax.init_stack(0x1000n);

// This function will be called before any "Mov" instruction is executed. There's also a hook_after_mnemonic function.
// It can be used to e.g. implement custom syscall handlers
ax.hook_before_mnemonic(Mnemonic.Mov, (instance) => {
    console.log("Executing a mov instruction");

    // Here you can e.g. modify registers, memory etc.
    instance.reg_write_64(Register.RCX, 0xabn);

    // this function *MUST* return one of
    // - instance.commit(): keep the changes we made in this handler and continue execution
    // - instance.stop(): keep changes, stop execution and return from the ax.execute() function
    // - instance.unchanged(): don't keep changes, continue execution

    // this will reset RCX to its previous value
    return instance.unchanged();
});

// Execute all instructions
await ax.execute();

// Log the final state of the emulator
console.log("Final state:", ax.toString());

// Prints "15"
console.log("RAX:", ax.reg_read_64(Register.RAX));
```

The emulator will just stop when reaching the end of the code.

</details>

### Emulate ELF binaries
The emulator also has some convenience functions for handling Linux/ELF binaries. The [test site](https://ax.010.one) uses these convenience functions to emulate programs.

<details>
<summary>Open for more info on how to emulate ELF files</summary>

One thing to note is that binaries usually exit via the `exit` syscall, which is not implemented by default (same as any other syscall). You must handle it yourself, like in the following example:

```js
let ax = Axecutor.from_binary(/* elf file content as Uint8Array */);

// Set up the stack according to the System V ABI.
// This sets up memory locations for command-line arguments and environment variables
// and writes the stack pointer to RSP
ax.init_stack_program_start(
  8n * 1024n, // Stack size
  ["/bin/my_binary", "arg1", "arg2"],
  [ /* environment variables */ ]
);


let syscallHandler = async function (ax: Axecutor) {
  let syscall_num = ax.reg_read_64(Register.RAX);
  let rdi = ax.reg_read_64(Register.RDI);
  let rsi = ax.reg_read_64(Register.RSI);
  let rdx = ax.reg_read_64(Register.RDX);

  console.log(`Syscall ${syscall_num} with args ${rdi}, ${rsi}, ${rdx}`);

  switch (syscall_num) {
    case 1n: {
      // WRITE syscall MUST write to stdout or stderr (stdin supported for compatibility)
      if (rdi != 0n && rdi != 1n && rdi != 2n) {
        throw new Error(`WRITE syscall: cannot write non-std{out,err} (!= 1,2) fds, but tried ${rdi}`);
      }

      // Read whatever was
      let result_buf = ax.mem_read_bytes(rsi, rdx);

      // Convert to string
      let result_str = new TextDecoder().decode(result_buf);

      // Do something with the string
      console.log("WRITE syscall:", result_str);

      // Return the number of bytes that were written in RAX,
      // that way the program knows it worked
      ax.reg_write_64(Register.RAX, rdx);

      return ax.commit();
    }
    case 60n: {
      // EXIT syscall
      console.log("EXIT syscall: exiting with code " + rdi);
      return ax.stop();
    }
  }

  throw `Unhandled syscall ${syscall_num}`;
}

// Register the syscall handler
ax.hook_before_mnemonic(Mnemonic.Syscall, syscallHandler);

// Execute the program
await ax.execute();
```

If your binaries need more syscalls, you can look at the [example site implementation](examples/web/src/components/Demo.vue) or get more details [here](https://syscalls.w3challs.com/?arch=x86_64).

</details>

## Contributing
If you want to contribute to this project, that's great! A good way to involved is using the emulator and finding things that could be improved :)
You could e.g. get started by adding support for a new instruction mnemonic. There's a tutorial on how to do that below.
If you run into problems setting up the development tools or have any other questions, feel free to open an issue.

The [`Makefile`](Makefile) has a lot of targets that can be useful for development. The `test` target runs tests both on your native machine and in WASM via NodeJS, making sure implemented instructions behave as expected in the target environment.

Please format the code using `make fmt` before submitting a pull request. Also make sure that `make precommit` passes.

### Development setup
1. Make sure you have installed Rust/Cargo, [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/), Node.js, NPM, Python, PIP, Make, GCC and the GNU Assembler
   - You can optionally install [mold](https://github.com/rui314/mold) to speed up link times (mostly for tests); the Makefile will automatically use it if it's installed
2. You should now be able to build the WebAssembly module with `make`
3. You can run `make dependencies` to install `cargo-watch`, `cargo-tarpaulin` (for generating test coverage info files) and python script dependencies
4. Try out running `cargo test` or `make watch-tests` to run tests
5. Run `make watch-debug` in one terminal to rebuild the WebAssembly module on changes, then run `make web` in another terminal to start the development server. You can also just run `make watch`, which runs both in the same terminal
6. Open the local example site and make changes! (link should be in the `make web` output)

#### How to implement a new mnemonic
The [`generate.py`](generate.py) script is used for generating instruction implementation stubs. You can e.g. run `python3 generate.py push` to generate a file for all instruction mnemonics that start with `push`; if you only want more exact matches use `push_` as argument. Note that you must have run a build for the WebAssembly package, as otherwise the script won't be able to find the files from the [`iced-x86` crate](https://crates.io/crates/iced-x86) that are used for generating the stubs.

Afterwards, run `make switch` to regenerate the instruction mnemonic switch statement (in [`src/instructions/generated.rs`](src/instructions/generated.rs)). Now your new stub functions are reachable.

Afterwards, it is recommended to automatically generate test cases, then implement the instruction. You can also add new [integration tests](src/instructions/integration_tests.rs) that use the instruction, this is especially important for instructions that operate on flags.

#### Automatically generate test cases
The repository comes with scripts for generating test cases for x86-64 instructions. They basically try out instructions on your real x86-64 CPU and extract the processor state change.

The test cases are generated by running e.g. `python3 t.py add al, [rbx]`, which tries around 6000 different inputs for the instruction (the extreme mode with `-e` tries around 100k inputs).
The generated test cases are deduplicated, resulting in only one test case per unique combination of flags that are set and cleared. Note that not necessarily all combinations will be discovered.
When generating tests, make sure that only the flags that are defined in the manual are tested for, e.g. for `imul` where only `CF` and `OF` are defined, you would pass `-f CF,OF` to the script.
For some instructions it makes more sense go by *result* instead of flags, you can do this by passing `-r` to the script.
For instructions with implicit arguments (e.g. `imul rax` also modifies `rdx`), you can pass `-i rdx` to the script to also test the implicit arguments (any comma separated list of operands should work).

Here is one of 19 test cases that was automatically discovered for `add al, [rbx]` (without `-e`):
```rust
// ax_test is macro that sets up the emulator, then runs setup and a post-execution assertion function
// add al, byte ptr [rbx]
ax_test![add_al_byte_ptr_rbx_cf;
    // The encoded instruction bytes:
    0x2, 0x3;
    // A setup function that is called before the instruction is executed:
    |a: &mut Axecutor| {
        write_reg_value!(b; a; AL; 0x20);

        // This sets up a memory area with a size of one byte, containing 0xff
        write_reg_value!(q; a; RBX; 0x1000);
        a.mem_init_zero(0x1000, 1).unwrap();
        a.mem_write_8(0x1000, 0xff).unwrap();
    };
    // This function is called after the instruction ran and checks the result:
    |a: Axecutor| {
        assert_reg_value!(b; a; AL; 0x1f);
        // Also make sure the source operand is unchanged
        assert_eq!(a.mem_read_8(0x1000).unwrap(), 0xff);
    };
    // On the left side of `;` are the flags that must be set after the instruction ran,
    // on the right are flags that must not be set
    (FLAG_CF; FLAG_PF | FLAG_ZF | FLAG_SF | FLAG_OF)
];
```

The test case generation script [`t.py`](t.py) currently only supports register, a subset of memory and immediate operands. It requires that the GNU Assembler `as` and `gcc` are installed and must be run on x86-64 Linux. It places thousands of generated assembly files and binaries in `/dev/shm/ax_*`, so in case you run out of RAM that is the place to check.

Another script for testing jumps ([`j.py`](j.py)) is also available, but it's not as automated. Some other convenience copy-paste texts can be generated with [`a.py`](a.py), e.g. with `python3 a.py add al, [rbx]` and then selecting `u` you'll get a code snippet for a JavaScript `Uint8Array` containing the bytes of the instruction.

If you want to adjust `t.py` for testing your own emulator, you should adjust the `__str__` method of the `TestCase` class to generate different syntax with the same information.

### Links
* [Intel x64 Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
* [AMD64 Developer Guides](https://developer.amd.com/resources/developer-guides-manuals/)

### Limitations
Here are some limitations that could be inspiration for future features:

* Only the Signed, Carry, Overflow, Zero and Parity status flags are supported
* Most instructions aren't implemented, especially
  * Anything with SSE registers
  * Anything I found too legacy
  * Many instructions
  * No support for anything related to segments
* Syscall and Interrupts are not implemented to spec.
  * If you have registered hooks using `hook_before_mnemonic` or `hook_after_mnemonic`) they are essentially a no-op with your handler executing
  * If no hooks are registered and a syscall/interrupt is executed, an exception is thrown
* The memory implementation is quite weird and needs an overhaul
  * Memory for code currently is basically its own implementation instead of just being stored using the normal memory implementation
  * Segments, access restrictions, page management etc. is missing
* ELF file parsing is currently really basic
  * Binaries with libc don't work due to relocations etc. not being implemented
  * Basically only very basic binaries work

### Ideas
See [issue 1](https://github.com/xarantolus/ax/issues/1) for some ideas for future features. Also feel free to open an issue if you have an idea :)


## [License](LICENSE)
GPL-3.0
