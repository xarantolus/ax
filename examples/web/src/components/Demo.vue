<template >
  <div class="middle width-2-3">
    <h1><a href="https://github.com/xarantolus/ax">AX Demo Site</a></h1>
    <p>
      This is the demo site for <a href="https://github.com/xarantolus/ax">ax, an x86-64 emulator</a>
      <template v-if="version && commit"> (v{{ version }}, <a :href="'https://github.com/xarantolus/ax/commit/' + commit">commit</a>)</template>.
    </p>
    <br />
    <p>
      Please select an ELF binary compiled with <code>-m64 -nostdlib -static</code> that only interacts with std{in,out,err}.
      Some binaries, especially the ones that use libc, might not work.
    </p>
    <div>
      <input type="file" ref="file">
      <button @click="runFile">Run!</button>
    </div>
    <br />
    <div v-if="programs.length > 0">
      Click to load one of the following binaries:
      <ul>
        <li v-for="program in programs" :key="program.name">
          <a :href="'/programs/' + program.binary" @click.prevent="setBinary(program.binary)">{{ program.name }}</a>: {{ program.description }} (<a :href="program_source_prefix + program.source_name" target="_blank">Source</a>)
        </li>
      </ul>
      <br />
      You can also download these binaries and run them on Linux. This of course shows that the emulator can run some real binaries without any modifications.
    </div>
    <br />
    <h2>Console output</h2>
    <Terminal ref="terminalRef" />
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import Terminal from './Terminal.vue';
import init, { Axecutor, Mnemonic, Register, version, commit } from 'ax-x86';

export default defineComponent({
  components: {
    Terminal,
  },
  async setup() {
    const terminalRef = ref<InstanceType<typeof Terminal>>();

    const termReset = () => {
      terminalRef.value?.term.reset();
      terminalRef.value?.term.clear();
      terminalRef.value?.term.focus();
    };

    onMounted(async () => {
      termReset();
      terminalRef.value?.term.writeln('Welcome to the AX test site! When you run a binary, the output will be shown here.');
    });

    const termWrite = (data: string | Uint8Array) => {
      terminalRef.value!.term.write(data);
    };

    const programs = [
      {
        name: "Exit Code",
        binary: "exit_code.bin",
        source_name: "exit/exit_code.s",
        description: "Exits with exit code 13",
      },
      {
        name: "Hello World",
        binary: "hello_world.bin",
        source_name: "hello_world/hello_world.s",
        description: "Prints \"Hello World!\" to stdout",
      },
      {
        name: "Alphabet",
        binary: "alphabet.bin",
        source_name: "alphabet/alphabet.s",
        description: "Prints the alphabet to stdout",
      },
      {
        name: "Input to uppercase",
        binary: "uppercase_naive.bin",
        source_name: "uppercase/uppercase_naive.s",
        description: "Repeatedly reads a single character from stdin and prints the uppercase representation",
      },
      {
        name: "Byte to hex",
        binary: "hex_naive.bin",
        source_name: "hex/hex_naive.s",
        description: "Repeatedly reads a single byte from stdin and prints the hex representation",
      },
      {
        name: "String length",
        binary: "strlen.bin",
        source_name: "strlen/strlen.s",
        description: "Reads a string from stdin and prints its length",
      },
      {
        name: "Print command-line arguments",
        binary: "args.bin",
        source_name: "args/args.S",
        description: "Prints all command-line arguments and environment variables",
      },
      /*
      {
        name: "Exit Code (C)",
        binary: "exit_c.bin",
        source_name: "exit/exit_c.c",
      },
      {
        name: "Hello C",
        binary: "hello_c.bin",
        source_name: "hello_c/hello_c.c",
      },
      {
        name: "thread_local errno (C)",
        binary: "thread_local.bin",
        source_name: "thread_local_c/thread_local.c",
      }
      */
    ]

    await init();

    return {
      terminalRef,
      termReset,
      termWrite,
      programs,
      program_source_prefix: "https://github.com/xarantolus/ax/blob/main/examples/programs/",
      version: version(),
      commit: commit(),
      brk_start: 0n,
      brk_len: 0n,
    }
  },
  methods: {
    toTwosComplement(num: bigint): bigint {
      return ~num + 1n;
    },

    async syscallHandler(ax: Axecutor) {
      let syscall_num = ax.reg_read_64(Register.RAX);
      let rdi = ax.reg_read_64(Register.RDI);
      let rsi = ax.reg_read_64(Register.RSI);
      let rdx = ax.reg_read_64(Register.RDX);

      console.log(`Syscall ${syscall_num} with args ${rdi}, ${rsi}, ${rdx}`);

      switch (syscall_num) {
        case 0n: {
          // READ syscall MUST read from stdin
          if (rdi != 0n) {
            throw new Error("READ syscall: cannot read from non-stdin (!= 0) fd, but tried " + rdi);
          }

          console.log("Reading " + rdx + " bytes from stdin");

          let inputBytes = [];
          let byte;
          do {
            byte = await this.terminalRef!.readByte();
            if (byte === undefined) {
              throw new Error("READ syscall: no input");
            }
            inputBytes.push(byte);
          } while (BigInt(inputBytes.length) < rdx && byte !== 0x0a);

          console.log(`Read ${inputBytes.length} bytes: ${inputBytes}, writing them to memory at ${rsi}`)

          ax.mem_write_bytes(rsi, new Uint8Array(inputBytes));
          ax.reg_write_64(Register.RAX, BigInt(inputBytes.length));

          return ax.commit();
        }
        case 1n: {
          // WRITE syscall MUST write to stdout or stderr
          // Actually this also supports also "writing" to stdin, as this also works in certain circumstances: https://stackoverflow.com/a/7680234
          if (rdi != 0n && rdi != 1n && rdi != 2n) {
            throw new Error(`WRITE syscall: cannot write non-std{out,err} (!= 1,2) fds, but tried ${rdi}`);
          }

          let result_buf = ax.mem_read_bytes(rsi, rdx);

          this.termWrite(result_buf);

          return ax.unchanged();
        }
        case 60n: {
          console.log("EXIT syscall: exiting with code " + rdi.toString(16));
          // EXIT syscall
          return ax.stop();
        }
        case 12n: {
          // brk syscall

          console.log("brk syscall: called with rdi = " + rdi.toString(16));

          // if called with 0 rdi for the first time, we should make up some memory, probably a 4k page
          if (rdi == 0n) {
            if (this.brk_start == 0n) {
              const len = 0x1000n;
              let start_addr = ax.init_zero_anywhere(len);
              this.brk_start = start_addr;
              this.brk_len = len;

              console.log("brk syscall: initialized memory at " + start_addr.toString(16) + " with length " + len.toString(16));
            }

            console.log("brk syscall: returning " + this.brk_start.toString(16) + " as brk start");
          } else {
            // called with a non-zero rdi, we should resize the memory
            let new_length = rdi - this.brk_start;
            if (new_length < 0n) {
              throw new Error("brk syscall: cannot resize memory to a negative length");
            }

            if (new_length < this.brk_len) {
              throw new Error("brk syscall: cannot resize memory to a smaller length");
            }

            console.log("brk syscall: resizing memory from " + this.brk_len.toString(16) + " to " + new_length.toString(16));

            ax.resize_section(this.brk_start, new_length);

            this.brk_len = new_length;
          }

          ax.reg_write_64(Register.RAX, this.brk_start + this.brk_len);

          return ax.commit();
        }
        case 158n: { // arch_prctl
          console.log("FS arch_prctl: operation 0x" + rdi.toString(16) + ", addr 0x" + rsi.toString(16))
          if (rdi === 0x1002n) {
            // ARCH_SET_FS
            console.log("Setting FS to " + rsi.toString(16));
            ax.write_fs(rsi);

            if (ax.read_fs() !== rsi) {
              throw "arch_prctl: failed to set FS";
            }

            console.log("Set FS to " + ax.read_fs().toString(16));
            ax.reg_write_64(Register.RAX, 0n);
            return ax.commit();
          }

          ax.reg_write_64(Register.RAX, this.toTwosComplement(-1n));
          return ax.commit();
        }
        case 102n: // getuid
        case 104n: // getgid
        case 107n: // geteuid
        case 108n: // getegid
          {
            // just return dummy values for these
            ax.reg_write_64(Register.RAX, 0n);
            return ax.commit();
          }
        default: {
          throw new Error("Syscall: unsupported RAX value " + syscall_num);
        }
      }
    },
    async setBinary(name: string) {
      try {
        this.termReset();

        this.termWrite(`Downloading ${name}...\n`);

        let response = await fetch(`/programs/${name}`);

        if (!response.ok) {
          this.termWrite("Error while fetching binary: " + response.statusText);
          return;
        }

        let content = await response.arrayBuffer().then(buf => new Uint8Array(buf));
        // Create a new file object from the binary and set it as the file input's value
        let file = new File([content], name, { type: "application/octet-stream" });
        let data = new DataTransfer();
        data.items.add(file);
        (this.$refs.file as any).files = data.files;

        this.termWrite(`Successfully downloaded ${name}, you can now run it.\n`);
      }
      catch (e) {
        this.termWrite("Error while fetching binary:\n" + e);
        return;
      }
    },
    async runFile() {
      this.termReset();

      this.brk_start = 0n;

      let ax;
      try {
        let files = (this.$refs.file as any).files as FileList;
        console.log(files);
        if (files.length != 1) {
          this.termWrite("Please select exactly one file");
          return;
        }
        let content = await files.item(0)?.arrayBuffer().then(buf => new Uint8Array(buf));
        if (!content) {
          this.termWrite("Failed to read file, please only use statically linked ELF binaries.");
          return;
        }
        ax = Axecutor.from_binary(content);
      }
      catch (e) {
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate);
        this.termWrite("Error while decoding binary:\n" + e);
        return;
      }
      try {
        ax.init_stack_program_start(8n * 1024n, ["/bin/my_binary", "arg1", "arg2"], [
          "USER=demo-user",
          "COLORTERM=truecolor",
          "TERM=xterm-256color",
        ]);
        ax.hook_before_mnemonic(Mnemonic.Syscall, this.syscallHandler);
      }
      catch (e) {
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate);
        this.termWrite("Error during initialisation:\n" + e + "\n\n" + axstate);
        return;
      }
      try {
        this.termWrite("Starting emulation...\n");
        await ax.execute();
        this.termWrite(`Program exited with exit code ${ax.reg_read_64(Register.RDI)}.\n`);
      }
      catch (e) {
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate);
        this.termWrite("\nError during execution:\n" + e + "\n\n" + axstate);
      }
    }
  }
})
</script>

<style>
.width-2-3 {
  width: 66.67%;
}

@media (max-width: 1440px) {
  .width-2-3 {
    width: 100%;
  }
}
</style>
