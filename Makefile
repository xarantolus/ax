.PHONY: build normal watch test clean

build:
	wasm-pack build --target=web --release

normal:
	wasm-pack build --target=web

watch:
	watchexec -r "make normal && ghfs"

test:
	cargo test

clean:
	rm -rf pkg
