<script setup lang="ts">
import { default as init, Axecutor, Mnemonic, Register } from 'ax';

await init();
</script>

<template>
  <h1>AX Test Site</h1>
  <div>
    <input type="file" ref="file">
    <button @click="runFile">Run!</button>
  </div>
  <pre id="console">{{console_content}}</pre>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  data() {
    return {
      console_content: "Content",
    };
  },
  methods: {
    async runFile() {
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
          alert("Failed to read file");
          return;
        }

        ax = Axecutor.from_binary(content);
        console.log("RIP: " + ax.reg_read_64(Register.RIP));

        ax.init_stack(8n * 1024n);

        ax.hook_before_mnemonic(Mnemonic.Ret, (ax: Axecutor) => {
          console.log("before RET @ " + ax.reg_read_64(Register.RIP));
        });

        ax.hook_after_mnemonic(Mnemonic.Ret, (ax: Axecutor) => {
          console.log("after RET @ " + ax.reg_read_64(Register.RIP));
        });

        ax.hook_before_mnemonic(Mnemonic.Syscall, (ax: Axecutor, mnemonic: number) => {
          console.log("Syscall!");

          let rax = ax.reg_read_64(Register.RAX);
          if (rax != 1n) {
            throw new Error("Unsupported RAX value in syscall, only write to stdout is supported in this demo");
          }

          let fd = ax.reg_read_64(Register.RDI);
          if (fd != 0n && fd != 1n) {
            throw new Error("Unsupported FD/RDI value in syscall, only write to stdout is supported in this demo");
          }

          let buf_ptr = ax.reg_read_64(Register.RSI);
          let buf_len = ax.reg_read_64(Register.RDX);

          let buf = ax.mem_read_bytes(buf_ptr, buf_len);
          let result_str = new TextDecoder().decode(buf);

          this.console_content += result_str;

          // No modifications made to axecutor, so return null
          return null;
        });

        await ax.execute();

        this.console_content += "\nFinished execution.";
      } catch (e) {
        let axstate = (ax ?? "no axecutor state available").toString();
        console.error(axstate)
        this.console_content += "\nError during execution:\n" + e + "\n\n" + axstate;
      }
    }
  }
})
</script>
