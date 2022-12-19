.PHONY: build debug watch test clean switch coverage fmt example-programs example copy-programs dependencies web build-web

MOLD_INSTALLED := $(shell which mold 2> /dev/null)
ifneq ($(MOLD_INSTALLED),)
  MOLD := mold -run
endif

build:
	$(MOLD) wasm-pack build --target web --release

debug:
	$(MOLD) wasm-pack build --target web --debug

example-programs:
	cd examples/programs && make build

watch:
	$(MOLD) cargo watch -s "make debug"

watch-tests:
	$(MOLD) cargo watch --why --exec 'tarpaulin --out Lcov --skip-clean --target-dir target/tests' --ignore lcov.info

web: copy-programs build
	$(MOLD) cd examples/web && npm install && npm run dev

build-web: copy-programs build
	$(MOLD) cd examples/web && npm install && npm run build

copy-programs: example-programs
	mkdir -p examples/web/public/programs
	cp -r $(shell find examples/programs -name "*.bin") examples/web/public/programs

fmt:
	$(MOLD) cargo fix --allow-staged && cargo fmt

coverage:
	$(MOLD) cargo tarpaulin --out Lcov --skip-clean

test:
	$(MOLD) cargo test

switch:
	py generate.py switch

dependencies:
	cargo install cargo-tarpaulin cargo-watch python-launcher
	py -m pip install pyperclip tqdm

clean:
	rm -rf pkg target examples/web/node_modules examples/web/dist
	cd examples/programs && make clean
