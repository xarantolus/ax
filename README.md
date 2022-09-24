# ax
Minimal x86-64 emulator for WebAssembly. Currently very WIP.

### Development setup
1. Make sure you have installed Rust/Cargo, Node.js and Make
2. [Install `wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
3. You should now be able to build the WebAssembly module with `make`
4. You can run `make dependencies` to install `cargo-watch`, `cargo-tarpaulin` (for generating test coverage info files) and python script dependencies
5. Try out running `cargo test` or `make watch-tests` to run tests
6. Now switch to the `example` directory and install web dependencies with `npm install`
7. Run `make watch` in one terminal to rebuild the WebAssembly module on changes, then run `npm run dev` in another terminal (in the `example` directory) to start the development server
8. Open the local example site and make changes! (link should be in the `npm run dev` output)

#### Generate mnemonic stubs
The [`generate.py`](generate.py) script is used for generating instruction implementation stubs. You can e.g. run `python3 generate.py push` to generate a file for all instruction mnemonics that start with `push`; if you only want more exact matches use `push_` as argument. Note that you must have run a build for the WebAssembly package, as otherwise the script won't be able to find the files from the [`icex-x86` crate](https://crates.io/crates/iced-x86) that are used for generating the stubs.

Afterwards, run `make generate` to regenerate the instruction mnemonic switch statement (in [`src/instructions/generated.rs`](src/instructions/generated.rs)). Now your new stub functions are reachable.

#### Automatically generate test cases
The repository comes with scripts for generating test cases for x86-64 instructions. The test cases are generated using e.g. `python3 t.py add al, [rbx]` (this tries around 6000 different inputs for the instruction, the extreme mode with `-e` tries around 100k inputs).
The generated test cases are deduplicated, resulting in only one test case per unique combination of flags that are set and cleared. Note that not necessarily all combinations will be discovered.

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

### Example
An example for using this WebAssembly module from JavaScript can be found in the [`example`](example) directory.

### Links
* [Intel x64 Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
* [AMD64 Developer Guides](https://developer.amd.com/resources/developer-guides-manuals/)


### Limitations
* No support for invalid instructions in the instruction stream
* Only the Signed, Carry, Overflow, Zero and Parity status flags are supported
* Most instructions aren't implemented, especially
  * Anything with SSE registers
  * Anything I found too legacy
  * Many instructions


### Ideas
* Look into parsing ELF files directly using something like https://crates.io/crates/object
