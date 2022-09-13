.PHONY: build clean

build:
	wasm-pack build --target=web --release

watch:
	watchexec -r "make && ghfs"

clean:
	rm -rf pkg
