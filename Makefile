.PHONY: build build-cjs debug watch test test-local test-node test-js test-scripts clean switch coverage fmt example-programs example copy-programs dependencies web build-web stats fmt python-dependencies ax generate docs watch-programs watch-debug watch-tests precommit clean-programs

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

######################
### WASM package builds
######################
build:
	$(MOLD) wasm-pack build --target web --release
RM_TARGETS += pkg

build-cjs:
	$(MOLD) wasm-pack build --target nodejs --release --out-dir pkg-cjs
RM_TARGETS += pkg-cjs

debug:
	$(MOLD) wasm-pack build --target web --debug

######################
### Binary builds
######################
bin: ax
ax:
	$(MOLD) cargo build --release && cp target/release/ax$(EXE_SUFFIX) .
RM_TARGETS += ax$(EXE_SUFFIX)


######################
### Web builds
######################
build-web: copy-programs build
	cd examples/web && npm install && npm run build
RM_TARGETS += examples/web/node_modules examples/web/dist .vite


######################
### Code generation and formatting
######################

# fmt will fail if switch or stats are not up to date, failing precommit
precommit: generate fmt test test-scripts all

# targets that might change files and thus prevent precommit from passing, especially when the version changes
generate: build-web switch stats test-js docs

stats:
	@$(PY) stats.py

docs: build
	cd js/docs && npm install && npm run build && npm run generate

switch:
	$(PY) generate.py switch

######################
### Tests & other dev jobs
######################
fmt:
	$(MOLD) cargo fix --allow-staged --all --all-features && \
	$(MOLD) cargo fmt --all && \
	$(MOLD) cargo clippy --all-targets --all-features --fix --allow-staged

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

watch:
	$(MAKE) -j3 web watch-debug watch-programs

watch-debug:
	$(MOLD) cargo watch -w src -s "$(MAKE) debug"

coverage:
	$(MOLD) cargo tarpaulin --out Lcov --skip-clean

watch-tests:
	$(MOLD) cargo watch -w src --why --exec 'tarpaulin --out Lcov --skip-clean --target-dir target/tests -- -q' --ignore lcov.info
RM_TARGETS += lcov.info

web: copy-programs build
	cd examples/web && npm install && npm run dev -- --host

test-scripts: python-dependencies
	$(PY) t.py --test
RM_TARGETS += __pycache__

######################
### Examples
######################
example-programs:
	cd examples/programs && $(MAKE) build

copy-programs: example-programs
	mkdir -p examples/web/public/programs
	cp -r $(shell find examples/programs -name "*.bin") examples/web/public/programs
RM_TARGETS += examples/web/public/programs

clean-programs:
	cd examples/programs && $(MAKE) clean

watch-programs:
	cd examples/programs && \
		cargo watch -w examples/programs -s "$(MAKE) clean-programs copy-programs"

######################
### Utilities
######################
dependencies: python-dependencies
	cargo install cargo-tarpaulin cargo-watch python-launcher

python-dependencies:
	$(PY) -m pip install pyperclip tqdm

clean:
	rm -rf $(RM_TARGETS)
	cd examples/programs && $(MAKE) clean
