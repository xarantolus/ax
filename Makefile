.PHONY: build normal watch test clean switch

build:
	wasm-pack build --target=web --release

normal:
	wasm-pack build --target=web

watch:
	cargo watch -s "make normal"

test:
	cargo test

switch:
	python3 generate.py switch

dependencies:
	python3 -m pip install pyperclip tqdm

clean:
	rm -rf pkg
