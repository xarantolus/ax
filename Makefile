.PHONY: build normal watch test clean generate

build:
	wasm-pack build --target web --release

normal:
	wasm-pack build --target web

watch:
	cargo watch -s "make normal"

test:
	cargo test

generate:
	python3 generate.py generate

dependencies:
	python3 -m pip install pyperclip tqdm

clean:
	rm -rf pkg
