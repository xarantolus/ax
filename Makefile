.PHONY: build build-cjs debug watch test test-local test-node test-js test-scripts clean switch coverage fmt example-programs example copy-programs dependencies web build-web stats fmt python-dependencies ax generate docs

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

RM_TARGETS := *.out Cargo.lock target

all: ax build build-cjs

build:
	$(MOLD) wasm-pack build --target web --release
RM_TARGETS += pkg

build-cjs:
	$(MOLD) wasm-pack build --target nodejs --release --out-dir pkg-cjs
RM_TARGETS += pkg-cjs

debug:
	$(MOLD) wasm-pack build --target web --debug

bin: ax
ax:
	$(MOLD) cargo build --release && cp target/release/ax$(EXE_SUFFIX) .
RM_TARGETS += ax$(EXE_SUFFIX)

# fmt will fail if switch or stats are not up to date
precommit: build-web switch stats fmt docs test test-scripts ax build

# targets that might change files and thus prevent precommit from passing, especially when the version changes
generate: build-web switch stats test-js

test-scripts: python-dependencies
	$(PY) t.py --test
RM_TARGETS += __pycache__

stats:
	@$(PY) stats.py

docs: build
	cd js/docs && npm install && npm run build && npm run generate

example-programs:
	cd examples/programs && $(MAKE) build

watch:
	$(MAKE) -j2 watch-debug web

watch-debug:
	$(MOLD) cargo watch -s "$(MAKE) debug"

watch-tests:
	$(MOLD) cargo watch --why --exec 'tarpaulin --out Lcov --skip-clean --target-dir target/tests' --ignore lcov.info
RM_TARGETS += lcov.info

web: copy-programs build
	cd examples/web && npm install && npm run dev

build-web: copy-programs build
	cd examples/web && npm install && npm run build
RM_TARGETS += examples/web/node_modules examples/web/dist .vite

copy-programs: example-programs
	mkdir -p examples/web/public/programs
	cp -r $(shell find examples/programs -name "*.bin") examples/web/public/programs
RM_TARGETS += examples/web/public/programs

fmt:
	$(MOLD) cargo fix --allow-staged --all --all-features && \
	$(MOLD) cargo fmt --all && \
	$(MOLD) cargo clippy --all-targets --all-features --fix --allow-staged

coverage:
	$(MOLD) cargo tarpaulin --out Lcov --skip-clean

test: test-local test-node test-js

test-local:
	@echo "Running tests on processor..."
	$(MOLD) cargo test

test-wasm: test-node

test-node:
	@echo "Running tests in Node/WASM..."
	wasm-pack test --node

test-js: build-cjs
	@echo "Testing JS API"
	cd js/test && npm install && npm test

switch:
	$(PY) generate.py switch

dependencies: python-dependencies
	cargo install cargo-tarpaulin cargo-watch python-launcher

python-dependencies:
	$(PY) -m pip install pyperclip tqdm

clean:
	rm -rf $(RM_TARGETS)
	cd examples/programs && $(MAKE) clean

