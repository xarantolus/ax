<template >
  <div class="middle width-2-3">
    <h1>AX Test Site</h1>
    <p>
      This is the demo site for [ax, an x86-64 emulator](github.com/xarantolus/ax).
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
    <div>
      You can also load one of the following binaries:
      <ul>
        <li><a href="/programs/hello_world.bin" @click.prevent="setBinary('hello_world.bin')">Hello World</a></li>
        <li><a href="/programs/alphabet.bin" @click.prevent="setBinary('alphabet.bin')">Alphabet</a></li>
        <li><a href="/programs/exit_code.bin" @click.prevent="setBinary('exit_code.bin')">Exit Code</a></li>
        <li><a href="/programs/uppercase_naive.bin" @click.prevent="setBinary('uppercase_naive.bin')">Uppercase</a></li>
        <li><a href="/programs/hex_naive.bin" @click.prevent="setBinary('hex_naive.bin')">Hex</a></li>
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
import { default as init, Axecutor, Mnemonic, Register } from 'ax';

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
      // Download emulator
      await init();

      termReset();
      terminalRef.value?.term.writeln('Welcome to the AX test site! When you run a binary, the output will be shown here.');
    });

    const termWrite = (data: string | Uint8Array) => {
      terminalRef.value!.term.write(data);
    };

    return {
      terminalRef,
      termReset,
      termWrite,
    }
  },
  methods: {
    async syscallHandler(ax: Axecutor) {
      let syscall_num = ax.reg_read_64(Register.RAX);
      let rdi = ax.reg_read_64(Register.RDI);
      let rsi = ax.reg_read_64(Register.RSI);
      let rdx = ax.reg_read_64(Register.RDX);

      switch (syscall_num) {
        case 0n: {
          // READ syscall MUST read from stdin
          if (rdi != 0n) {
            throw new Error("READ syscall: cannot read from non-stdin (!= 0) fd, but tried " + rdi);
          }

          console.log("Reading " + rdx + " bytes from stdin");

          let inputBytes = [];
          do {
            let byte = await this.terminalRef!.readByte();
            if (byte === undefined) {
              throw new Error("READ syscall: no input");
            }
            inputBytes.push(byte);
          } while (BigInt(inputBytes.length) < rdx);

          console.log(`Read ${inputBytes.length} bytes: ${inputBytes}, writing them to memory at ${rsi}`)

          ax.mem_write_bytes(rsi, new Uint8Array(inputBytes));
          ax.reg_write_64(Register.RAX, BigInt(inputBytes.length));

          return ax.commit();
        }
        case 1n: {
          // WRITE syscall MUST write to stdout or stderr
          // Actually this also supports also "writing" to stdin, as this also works in certain circumstances: https://stackoverflow.com/a/7680234
          if (rdi != BigInt(0) && rdi != BigInt(1) && rdi != BigInt(2)) {
            throw new Error(`WRITE syscall: cannot write non-std{out,err} (!= 1,2) fds, but tried ${rdi}`);
          }

          let result_buf = ax.mem_read_bytes(rsi, rdx);

          this.termWrite(result_buf);

          return ax.unchanged();
        }
        case 60n: {
          // EXIT syscall
          return ax.stop();
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
        ax.init_stack(8n * 1024n);
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
