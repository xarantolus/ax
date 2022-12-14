.PHONY: build debug watch test test-local test-node test-scripts clean switch coverage fmt example-programs example copy-programs dependencies web build-web stats fmt python-dependencies

MOLD_INSTALLED := $(shell which mold 2> /dev/null)
ifneq ($(MOLD_INSTALLED),)
  MOLD := mold -run
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

build:
	$(MOLD) wasm-pack build --target web --release

debug:
	$(MOLD) wasm-pack build --target web --debug

# fmt will fail if switch or stats are not up to date
precommit: build-web switch stats fmt test test-scripts build

test-scripts: python-dependencies
	$(PY) t.py --test

stats:
	@$(PY) stats.py

example-programs:
	cd examples/programs && make build

watch:
	make -j2 watch-debug web

watch-debug:
	$(MOLD) cargo watch -s "make debug"

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

test: test-node test-local

test-local:
	@echo "Running tests on processor..."
	$(MOLD) cargo test

test-wasm: test-node

test-node:
	@echo "Running tests in Node/WASM..."
	wasm-pack test --node

switch:
	$(PY) generate.py switch

dependencies: python-dependencies
	cargo install cargo-tarpaulin cargo-watch python-launcher

python-dependencies:
	@$(PY) -m pip install --quiet pyperclip tqdm

clean:
	rm -rf pkg target examples/web/node_modules examples/web/dist .vite
	cd examples/programs && make clean
	rm -f lcov.info

