.PHONY: build build-cjs debug watch test test-local test-node test-js test-scripts clean switch coverage fmt example-programs example copy-programs dependencies web build-web stats fmt python-dependencies ax

MOLD_INSTALLED := $(shell which mold 2> /dev/null)
ifneq ($(MOLD_INSTALLED),)
  MOLD := mold -run
endif

EXE_SUFFIX:=
ifeq ($(OS),Windows_NT)
  EXE_SUFFIX:=.exe
endif

PY_INSTALLED := $(shell which py 2> /dev/null)
ifeq ($(PY_INSTALLED),)
	ifeq ($(shell which python3 2> /dev/null),)
		PY := python
	else
		PY := python3
	endif
else
  PY := py -3
endif

all: ax build build-cjs

build:
	$(MOLD) wasm-pack build --target web --release

build-cjs:
	$(MOLD) wasm-pack build --target nodejs --release --out-dir pkg-cjs

debug:
	$(MOLD) wasm-pack build --target web --debug

bin: ax
ax:
	$(MOLD) cargo build --release && cp target/release/ax$(EXE_SUFFIX) .

# fmt will fail if switch or stats are not up to date
precommit: build-web switch stats fmt test test-scripts ax build

test-scripts: python-dependencies
	$(PY) t.py --test

stats:
	@$(PY) stats.py

example-programs:
	cd examples/programs && $(MAKE) build

watch:
	$(MAKE) -j2 watch-debug web

watch-debug:
	$(MOLD) cargo watch -s "$(MAKE) debug"

watch-tests:
	$(MOLD) cargo watch --why --exec 'tarpaulin --out Lcov --skip-clean --target-dir target/tests' --ignore lcov.info

web: copy-programs build
	cd examples/web && npm install && npm run dev

build-web: copy-programs build
	cd examples/web && npm install && npm run build

copy-programs: example-programs
	mkdir -p examples/web/public/programs
	cp -r $(shell find examples/programs -name "*.bin") examples/web/public/programs

fmt:
	$(MOLD) cargo fix --allow-staged --all --all-features && \
	$(MOLD) cargo fmt --all && \
	$(MOLD) cargo clippy --all-targets --all-features --fix --allow-staged

coverage:
	$(MOLD) cargo tarpaulin --out Lcov --skip-clean

test: test-local test-node

test-local:
	@echo "Running tests on processor..."
	$(MOLD) cargo test

test-wasm: test-node

test-node:
	@echo "Running tests in Node/WASM..."
	wasm-pack test --node

test-js: build-cjs
	@echo "Testing JS API"
	cd js_test && npm install && npm test

switch:
	$(PY) generate.py switch

dependencies: python-dependencies
	cargo install cargo-tarpaulin cargo-watch python-launcher

python-dependencies:
	@$(PY) -m pip install --quiet pyperclip tqdm

clean:
	rm -rf pkg pkg-cjs target examples/web/node_modules examples/web/dist examples/web/public/programs .vite
	cd examples/programs && $(MAKE) clean
	rm -f lcov.info

