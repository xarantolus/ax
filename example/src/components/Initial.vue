<script setup lang="ts">
import { default as init, Axecutor, Mnemonic, Register } from 'ax';

await init();

await (async function x() {
  // mov rax, 5; sub rax, 5
  let code = new Uint8Array([0x48, 0xc7, 0xc0, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xe8, 0x5]);

  let ex = new Axecutor(code, 0x1n, 0x1n);
  ex.mem_init_zero(0x0n, 0x1000n);

  console.log(ex.toString());

  ex.hook_before_mnemonic(Mnemonic.Sub, (ax: Axecutor, mnemonic: number) => {
    // let ax = this as unknown as Axecutor;

    console.log("Before mnemonic", ax, mnemonic);
    console.log(ax.reg_read_64(Register.RAX));
  });

  ex.hook_after_mnemonic(Mnemonic.Sub, (ax: Axecutor, mnemonic: number) => {
    console.log("After mnemonic", ax, mnemonic);
    console.log(ax.reg_read_64(Register.RAX));

    ax.mem_write_64(0x1n, 0x1n);
    console.log("wrote to rax");
  });

  console.log(ex.toString());

  while (await ex.step()) {
    console.log(ex.toString());
  }

  console.log("Final state:\n" + ex.toString());

  console.log("rax", ex.reg_read_64(Register.RAX));
})();
</script>

<template>
  <div>Test</div>
</template>
