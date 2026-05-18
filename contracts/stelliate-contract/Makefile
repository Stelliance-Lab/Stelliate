# Build the contract WASM (reference-types must be disabled for Soroban compatibility)
build:
	RUSTFLAGS="-C target-feature=-reference-types" \
	cargo build --target wasm32-unknown-unknown --release

# Strip and optimise the WASM for deployment
opt: build
	wasm-opt \
		target/wasm32-unknown-unknown/release/stelliate_contract.wasm \
		--disable-reference-types \
		-Oz \
		-o target/wasm32-unknown-unknown/release/stelliate_contract_opt.wasm

# Deploy optimised WASM to testnet
deploy: opt
	stellar contract deploy \
		--wasm target/wasm32-unknown-unknown/release/stelliate_contract_opt.wasm \
		--source deployer \
		--network testnet

test:
	cargo test

.PHONY: build opt deploy test
