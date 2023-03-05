<template >
  <div v-if="error" class="middle width-2-3">
    <h1>Error</h1>
    <p>
      An error occurred while initializing this page:
    </p>
    <pre>{{ error }}</pre>
    <p>
      Please make sure that your browser supports WebAssembly.
    </p>
    <p>
      To just see the source code, <a href="https://github.com/xarantolus/ax">visit the Git repository</a>.
    </p>
    <p>
      For more information about the emulator, see the <a href="https://ax.010.one/docs/">documentation</a>.
    </p>
  </div>
  <div v-else class="middle width-2-3">
    <h1><a href="https://github.com/xarantolus/ax">AX Demo Site</a></h1>
    <p>
      This is the demo site for <a href="https://github.com/xarantolus/ax">ax, an x86-64 emulator</a>
      <template v-if="version && commit"> (v{{ version }}, <a :href="'https://github.com/xarantolus/ax/commit/' + commit">commit</a>)</template>.
    </p>
    <p>
      To use it in your own projects, install the <a href="https://www.npmjs.com/package/ax-x86"><code>ax-x86</code> npm package</a>:
    <div class="install">
      <pre>npm install ax-x86</pre>
    </div>
    </p>
    <p>See the <a href="https://ax.010.one/docs/">documentation</a> for more information on how to use the package.</p>
    <br />
    <h2>Try it out</h2>
    <p>
      Here you can select an ELF binary compiled with <code>-m64 -nostdlib -static</code> that only interacts with std{in,out,err}.
      Some binaries, especially the ones that use libc, might not yet work.
    </p>
    <div class="mt-1">
      <input :disabled="is_running" type="file" ref="file">
      <button :disabled="is_running" @click="runFile">Run!</button>
    </div>
    <br />
    <div v-if="programs!.length > 0">
      Alternatively, you can load one of the following binaries by just clicking:
      <ul>
        <li v-for="program in programs" :key="program.name">
          <a :href="'/programs/' + program.binary" @click.prevent="setBinary(program.binary)">{{ program.name }}</a>: {{ program.description }} (<a :href="program_source_prefix + program.source_name" target="_blank">Source</a>)
        </li>
      </ul>
      These exact binaries can also be run on Linux. This shows that the emulator can run some real binaries without any modifications.
    </div>
    <div>
      <h3 class="mt-1">Console output</h3>
      <button class="reload-button" @click="reload" :disabled="!is_running">Cancel running binary</button>
    </div>
    <Terminal ref="terminalRef" class="mt-1" />
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import Terminal from './Terminal.vue';
import init, { Axecutor, Mnemonic, Register, Syscall, version, commit } from 'ax-x86';

export default defineComponent({
  components: {
    Terminal,
  },
  async setup() {
    try {
      const terminalRef = ref<InstanceType<typeof Terminal>>();

      const termReset = () => {
        terminalRef.value?.term.reset();
        terminalRef.value?.term.clear();
        terminalRef.value?.term.focus();
      };

      onMounted(async () => {
        termReset();
        terminalRef.value?.term.writeln('Welcome to the AX demo site! When you run a binary, the output will be shown here.');
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
        {
          name: "Hello World (C, -nostdlib)",
          binary: "hello_world_c_nostdlib.bin",
          source_name: "hello_world_c_nostdlib/hello_world_c_nostdlib.c",
          description: "Prints \"Hello World!\" to stdout",
        },
        {
          name: "Fibonacci (C, -nostdlib)",
          binary: "fib_c_nostdlib.bin",
          source_name: "fib_c_nostdlib/fib_c_nostdlib.c",
          description: "Prints Fibonacci numbers to stdout",
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
        },
        {
          name: "thread_local errno (C)",
          binary: "thread_local.bin",
          source_name: "thread_local_c/thread_local.c",
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
        brk_start: 0n,
        brk_len: 0n,
        is_running: ref(false),
        error: ref<Error | null>(null),
      }
    } catch (e) {
      return {
        error: e,
      }
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

          this.termWrite!(result_buf);

          return ax.unchanged();
        }
        // TODO: make sure that all syscalls return something sensible, the following might not
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
      if (this.is_running) {
        alert("Cannot change binary while another program is running");
        return;
      }
      try {
        this.termReset!();

        this.termWrite!(`Downloading ${name}...\n`);

        let response = await fetch(`/programs/${name}`);

        if (!response.ok) {
          this.termWrite!("Error while fetching binary: " + response.statusText);
          return;
        }

        let content = await response.arrayBuffer().then(buf => new Uint8Array(buf));
        // Create a new file object from the binary and set it as the file input's value
        let file = new File([content], name, { type: "application/octet-stream" });
        let data = new DataTransfer();
        data.items.add(file);
        (this.$refs.file as any).files = data.files;

        this.termWrite!(`Successfully downloaded ${name}, you can now run it.\n`);
      }
      catch (e) {
        this.termWrite!("Error while fetching binary:\n" + e);
        return;
      }
    },
    async runFile() {
      if (this.is_running) {
        alert("Cannot run program while another program is running");
        return;
      }

      this.termReset!();

      this.brk_start = 0n;
      this.is_running = true;

      let ax;
      try {
        let files = (this.$refs.file as any).files as FileList;
        console.log(files);
        if (files.length != 1) {
          this.termWrite!("Please select exactly one file\n");
          this.is_running = false;
          return;
        }
        let content = await files.item(0)?.arrayBuffer().then(buf => new Uint8Array(buf));
        if (!content) {
          this.termWrite!("Failed to read file, please only use statically linked ELF binaries.\n");
          this.is_running = false;
          return;
        }
        ax = Axecutor.from_binary(content);
      }
      catch (e) {
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate);
        this.termWrite!("Error while decoding binary:\n" + e);
        this.is_running = false;
        return;
      }
      try {
        ax.init_stack_program_start(8n * 1024n, ["/bin/my_binary", "arg1", "arg2"], [
          "USER=demo-user",
          "COLORTERM=truecolor",
          "TERM=xterm-256color",
        ]);
        ax.handle_syscalls(Syscall.Exit, Syscall.Brk, Syscall.ArchPrctl, Syscall.Pipe);
        ax.hook_before_mnemonic(Mnemonic.Syscall, this.syscallHandler);
      }
      catch (e) {
        console.error(e);
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate);
        this.termWrite!("Error during initialisation:\n" + e + "\n\n" + axstate);
        this.is_running = false;
        return;
      }
      try {
        this.termWrite!("Starting emulation...\n");
        await ax.execute();
        this.termWrite!(`Program exited with exit code ${ax.reg_read_64(Register.RDI)}.\n`);
      }
      catch (e) {
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate);
        this.termWrite!("\nError during execution:\n" + e + "\n\n" + axstate);
      }

      this.is_running = false;
    },
    reload() {
      window.location.reload();
    },
  }
})
</script>

<style>
.install {
  padding: 16px;
  border-radius: 16px;
  overflow: auto;
  background-color: var(--vt-c-black-soft);
}

.install>pre::before {
  content: "$ ";
}

button,
input {
  border-radius: 8px;
  padding: 4px;
  min-width: 10%;
  max-width: 40%;
}

.width-2-3 {
  width: 66.67%;
}

.mt-1 {
  margin-top: 1%;
}

@media (max-width: 1440px) {
  .width-2-3 {
    width: 100%;
  }
}

.reload-button {
  position: absolute;
  top: 0;
  right: 0;
  color: var(--vt-c-white);
  background-color: var(--vt-c-black-soft);
  border: none;
  font-size: 1.25em;
  cursor: pointer;
}

.reload-button:disabled {
  cursor: not-allowed;
  opacity: 0.5;
}
</style>
