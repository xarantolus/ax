<template >
  <div class="middle width-2-3">
    <h1>AX Test Site</h1>
    <p>
      This is the demo site for <a href="https://github.com/xarantolus/ax">ax, an x86-64 emulator</a>
      <template v-if="version && commit"> (v{{ version }}, <a :href="'https://github.com/xarantolus/ax/commit/' + commit">commit</a>)</template>.
    </p>
    <br />
    <p>
      Please select an ELF binary compiled with <code>-m64 -nostdlib -static</code> that only interacts with std{in,out,err}.
      Some binaries (especially the ones that use libc) will not work due to the ELF loader being very basic right now.
    </p>
    <div>
      <input type="file" ref="file">
      <button @click="runFile">Run!</button>
    </div>
    <br />
    <div v-if="programs.length > 0">
      You can also load one of the following binaries:
      <ul>
        <li v-for="program in programs" :key="program.name">
          <a :href="'/programs/' + program.binary" @click.prevent="setBinary(program.binary)">{{ program.name }}</a> (<a :href="program_source_prefix + program.source_name" target="_blank">Source</a>)
        </li>
      </ul>
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
import { default as init, Axecutor, Mnemonic, Register, version, commit } from 'ax-x86';

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
        name: "Hello World",
        binary: "hello_world.bin",
        source_name: "hello_world/hello_world.s",
      },
      {
        name: "Alphabet",
        binary: "alphabet.bin",
        source_name: "alphabet/alphabet.s",
      },
      {
        name: "Exit Code",
        binary: "exit_code.bin",
        source_name: "exit/exit_code.s",
      },
      {
        name: "Input to uppercase",
        binary: "uppercase_naive.bin",
        source_name: "uppercase/uppercase_naive.s",
      },
      {
        name: "Byte to hex",
        binary: "hex_naive.bin",
        source_name: "hex/hex_naive.s",
      },
      {
        name: "String length",
        binary: "strlen.bin",
        source_name: "strlen/strlen.s",
      },
      {
        name: "Exit Code (C)",
        binary: "exit_c.bin",
        source_name: "exit/exit_c.c",
      },
      {
        name: "Hello C",
        binary: "hello_c.bin",
        source_name: "hello_c/hello_c.c",
      }
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
      brk: 0n,
    }
  },
  methods: {
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
        case 12n: {
          // brk syscall
          // if called with 0 rdi for the first time, we should make up some memory, probably a 4k page
          if (rdi == 0n) {
            if (this.brk == 0n) {
              this.brk = ax.init_zero_anywhere(0x1000n);
            }

            ax.reg_write_64(Register.RAX, this.brk);

            return ax.commit();
          }

          // Resizing memory is currently not supported
          throw new Error("brk syscall: resizing memory is not supported");
        }
        case 60n: {
          // EXIT syscall
          return ax.stop();
        }
        case 102n: // getuid
        case 104n: // getgid
        case 107n: // geteuid
        case 108n: // getegid
        case 158n: // arch_prctl
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

      this.brk = 0n;

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
        ax.init_stack_program_start(8n * 1024n, ["/bin/my_binary", "arg1", "arg2"]);
        let rsp = ax.reg_read_64(Register.RSP);
        ax.reg_write_64(Register.RSP, rsp - 1024n);
        ax.hook_before_mnemonic(Mnemonic.Ret, (ax: Axecutor) => {
          console.log("before RET @ " + ax.reg_read_64(Register.RIP));
          return ax.unchanged();
        });
        ax.hook_after_mnemonic(Mnemonic.Ret, (ax: Axecutor) => {
          console.log("after RET @ " + ax.reg_read_64(Register.RIP));
          return ax.unchanged();
        });
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
