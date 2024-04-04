rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

build:
	cargo build --quiet

test:
	cargo test --quiet

run:
	cargo run 

release:
	cargo build --release

release-arm:
	cargo build --release --arm64

all: format lint test 