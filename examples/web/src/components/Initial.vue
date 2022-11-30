<script setup lang="ts">
import { default as init, Axecutor, Mnemonic, Register } from 'ax';

await init();
</script>

<template >
  <div class="middle width-2-3">
    <h1>AX Test Site</h1>
    <p>Select an ELF binary compiled with <code>-m64 -nostdlib -static</code> that only interacts with std{in,out,err}.</p>
    <div>
      <input type="file" ref="file">
      <button @click="runFile">Run!</button>
    </div>
    <pre id="console">{{console_content}}</pre>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  data() {
    return {
      console_content: "This is the console output. It will contain stdout/stderr messages.",
    };
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
            throw new Error('READ syscall: cannot read from non-stdin (!= 0) fd, but tried ' + rdi);
          }

          console.log("Reading " + rdx + " bytes from stdin");

          let inputBytes;
          do {
            let input = prompt("Please input " + rdx + " characters");
            if (input == null) {
              throw new Error('READ syscall: no input');
            }
            inputBytes = new TextEncoder().encode(input);
          } while (BigInt(inputBytes.length) != rdx);

          ax.mem_write_bytes(rsi, inputBytes);

          return ax.commit();
        }
        case 1n: {
          // WRITE syscall MUST write to stdout or stderr
          // Actually this also supports also "writing" to stdin, as this also works in certain circumstances: https://stackoverflow.com/a/7680234
          if (rdi != BigInt(0) && rdi != BigInt(1) && rdi != BigInt(2)) {
            throw new Error(`WRITE syscall: cannot write non-std{out,err} (!= 1,2) fds, but tried ${rdi}`);
          }

          let result_buf = ax.mem_read_bytes(rsi, rdx);
          let result_str = new TextDecoder().decode(result_buf);

          console.log("Writing " + rdx + " bytes to stdout/stderr: " + result_str);

          this.console_content += result_str;

          return ax.unchanged();
        }
        case 60n: {
          // EXIT syscall
          console.log("Exiting with code " + rdi);
          return ax.stop();
        }
        default: {
          throw new Error('Syscall: unsupported RAX value ' + syscall_num);
        }
      }
    },
    async runFile() {
      this.console_content = "";

      let ax;

      try {
        let files = (this.$refs.file as any).files as FileList;
        console.log(files);

        if (files.length != 1) {
          alert("Please select exactly one file");
          return;
        }

        let content = await files.item(0)?.arrayBuffer().then(buf => new Uint8Array(buf));
        if (!content) {
          alert("Failed to read file, please only use statically linked ELF binaries.");
          return;
        }

        ax = Axecutor.from_binary(content);
      } catch (e) {
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate)
        this.console_content = "Error while decoding binary:\n" + e;
        return;
      }

      try {
        ax.init_stack(8n * 1024n);

        ax.hook_before_mnemonic(Mnemonic.Ret, (ax: Axecutor) => {
          console.log("before RET @ " + ax.reg_read_64(Register.RIP));

          return ax.unchanged();
        });

        ax.hook_after_mnemonic(Mnemonic.Ret, (ax: Axecutor) => {
          console.log("after RET @ " + ax.reg_read_64(Register.RIP));

          return ax.unchanged();
        });

        ax.hook_before_mnemonic(Mnemonic.Syscall, this.syscallHandler);

        await ax.execute();

        this.console_content += `Program exited with exit code ${ax.reg_read_64(Register.RDI)}.`;
      } catch (e) {
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate)
        this.console_content += "\nError during execution:\n" + e + "\n\n" + axstate;
      }
    }
  }
})
</script>

<style>
.width-2-3 {
  width: 66.67%;
}
</style>
