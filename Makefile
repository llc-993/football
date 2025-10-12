.PHONY: help build test check fmt clippy clean run-business run-manage

help:
	@echo "可用命令:"
	@echo "  make build         - 构建所有项目"
	@echo "  make test          - 运行所有测试"
	@echo "  make check         - 检查代码"
	@echo "  make fmt           - 格式化代码"
	@echo "  make clippy        - 运行 clippy 检查"
	@echo "  make clean         - 清理构建产物"
	@echo "  make run-business  - 运行 business 服务"
	@echo "  make run-manage    - 运行 manage 服务"

build:
	cargo build

build-release:
	cargo build --release

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

clean:
	cargo clean

run-business:
	cargo run --bin football-business

run-manage:
	cargo run --bin football-manage

dev-business:
	RUST_LOG=debug cargo run --bin football-business

dev-manage:
	RUST_LOG=debug cargo run --bin football-manage

