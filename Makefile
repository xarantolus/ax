.PHONY: build debug watch test clean switch coverage fmt example-programs example copy-programs dependencies web build-web

build:
	wasm-pack build --target web --release

debug:
	wasm-pack build --target web --debug

example-programs:
	cd examples/programs && make build

watch:
	cargo watch -s "make debug"

watch-tests:
	cargo watch --why --exec 'tarpaulin --out Lcov --skip-clean --target-dir target/tests' --ignore lcov.info

web: copy-programs build
	cd examples/web && npm install && npm run dev

build-web: copy-programs build
	cd examples/web && npm install && npm run build

copy-programs: example-programs
	mkdir -p examples/web/public/programs
	cp -r $(shell find examples/programs -name "*.bin") examples/web/public/programs

fmt:
	cargo fix --allow-staged && cargo fmt

coverage:
	cargo tarpaulin --out Lcov --skip-clean

test:
	cargo test

switch:
	py generate.py switch

dependencies:
	cargo install cargo-tarpaulin cargo-watch python-launcher
	py -m pip install pyperclip tqdm

clean:
	rm -rf pkg target examples/web/node_modules examples/web/dist
	cd examples/programs && make clean
