.PHONY: build clean

build:
	wasm-pack build --target=web --release

normal:
	wasm-pack build --target=web

watch:
	watchexec -r "make normal && ghfs"

clean:
	rm -rf pkg
