.PHONY: install
install:
	@cargo build

clean:
	@cargo clean

.PHONY: test
test:
	@cargo nextest run --no-fail-fast --run-ignored all --verbose --color always

.PHONY: f
f:
	rustfmt $(shell find src benches examples -name "*.rs" -type f)

.PHONY: fmt
fmt:
	@cargo fmt --all --

.PHONY: fmt-check
fmt-check:
	@cargo fmt --all --  --check

lint:
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: build-dev
build-dev:
	@cargo build

.PHONY: build-rel
build-rel:
	@cargo build --release

.PHONY: run-server
run-server:
	RUST_BACKTRACE=full CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true ENV=dev RUST_LOG=info cargo run --bin orderbook-api-server -- --config $(PWD)/config.toml

.PHONY: docker-build
docker-build:
	DOCKER_BUILDKIT=1 docker build --file Dockerfile --tag orderbook-api --target release --build-arg BUILD_ENV=release --build-arg RUST_VERSION=stable --build-arg RUSTC_WRAPPER="sccache" .

.PHONY: doc
doc:
	@cargo doc --no-deps

.PHONY: build-dev test doc fmt-check fmt lint 