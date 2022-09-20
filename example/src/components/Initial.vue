<script setup lang="ts">
import { default as init, Axecutor, Mnemonic, Register } from 'ax';

await init();


// mov rax, 5; sub rax, 5
let code = new Uint8Array([0x48, 0xc7, 0xc0, 0x5, 0x0, 0x0, 0x0, 0x48, 0x83, 0xe8, 0x5]);

let ex = new Axecutor(code, 0x1n, 0x1n);

ex.hook_before_mnemonic(Mnemonic.Sub, (mnemonic: number) => {
  let ax = this as unknown as Axecutor;
  console.log(this, ax);

  console.log("Passed mnemonic", mnemonic);
  // console.log("RAX:", ex.reg_read_32(Register.EAX));
});

console.log(ex.toString());

while (await ex.step()) {
  // console.log(ex.toString());
}

console.log("Final state:\n" + ex.toString());

</script>

<template>
  <div>Test</div>
</template>
