<template>
	<div class="terminal" ref="terminalHTMLElement"></div>
</template>

<script lang="ts">
import { defineComponent, onMounted, onUnmounted, ref, type Ref } from 'vue';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import 'xterm/css/xterm.css';

export default defineComponent({
	name: 'Terminal',
	setup(props, context) {
		// Setup xterm with automatic resize
		const terminalHTMLElement: Ref<HTMLElement | null> = ref(null);
		const term: Terminal = new Terminal({
			convertEol: true,
		});
		const fitAddon: FitAddon = new FitAddon();
		term.loadAddon(fitAddon);

		let resizeHandler = () => {
			fitAddon.fit();
		};

		let termBuffer = new Uint8Array();
		let inputBufferEventElement = document.createElement("div");
		let inputAllowed = false;

		let mergeBuffers = (arrayOne: Uint8Array, arrayTwo: Uint8Array) => {
			if (arrayOne.length === 0) return arrayTwo;
			let mergedArray = new Uint8Array(arrayOne.length + arrayTwo.length);
			mergedArray.set(arrayOne);
			mergedArray.set(arrayTwo, arrayOne.length);
			return mergedArray;
		};

		let keyInputHandler = (input: { key: string; domEvent: KeyboardEvent; }, _: void) => {
			// Length is 1 for printable characters, e.g. "a", "ö" etc; however not for special keys
			if (!inputAllowed || input.key.length !== 1) return;


			let inputBytes = new TextEncoder().encode(input.key.replace("\r", "\n"));
			if (inputBytes.length === 0) return;

			// Echo back what was put in
			term.write(inputBytes);

			termBuffer = mergeBuffers(termBuffer, inputBytes);

			if (termBuffer.length === 0) return;

			// Only trigger the event if there is a newline in the buffer, just like in most terminals
			if (termBuffer.includes('\n'.charCodeAt(0))) {
				const event = new Event('buffer');
				inputBufferEventElement.dispatchEvent(event);
			}
		}

		onMounted(() => {
			if (!terminalHTMLElement.value) throw new Error("Terminal HTML element not found");

			term.open(terminalHTMLElement.value);
			window.addEventListener('resize', resizeHandler);
			resizeHandler();

			term.onKey(keyInputHandler);
		});

		term.attachCustomKeyEventHandler((arg) => {
			if (arg.ctrlKey && arg.code === "KeyC" && arg.type === "keydown") {
				const selection = term.getSelection();
				if (selection) {
					navigator.clipboard.writeText(selection);
					return false;
				}
			}
			return true;
		});

		let readByte = () => {
			let readFirstBufferByte = () => {
				let byte = termBuffer[0];
				termBuffer = termBuffer.slice(1);
				return Promise.resolve(byte);
			}

			console.log("readByte", termBuffer);
			if (termBuffer.length > 0) {
				console.log("reading from buffer");
				return readFirstBufferByte();
			}

			let setInputAllowed = (allowed: boolean) => {
				inputAllowed = allowed;
				term.options.cursorBlink = allowed;
			}

			console.log("waiting for input");
			setInputAllowed(true);

			// The next time something is put in the buffer we return it
			return new Promise<number>((resolve) => {
				inputBufferEventElement.addEventListener('buffer', async function () {
					setInputAllowed(false);
					resolve(readFirstBufferByte());
				}, {
					once: true,
				});
			})
		}

		context.expose({
			term,
			readByte,
		});

		onUnmounted(() => {
			window.removeEventListener('resize', resizeHandler);
			term.dispose();
		});

		return {
			terminalHTMLElement,
			term,
			readByte,
		};
	},
})

</script>

<style scoped>
.terminal {
	text-align: left;
	width: 100%;
	height: 100%;
}
</style>

<style>
.terminal-container {
	/* this is important */
	overflow: hidden;
}

.xterm .xterm-viewport {
	/* see : https://github.com/xtermjs/xterm.js/issues/3564#issuecomment-1004417440 */
	width: initial !important;
}
</style>
