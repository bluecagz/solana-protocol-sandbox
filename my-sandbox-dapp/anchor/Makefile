PROGRAM_SO=target/deploy/mysandboxdapp.so

# Generate a new keypair and configure Solana CLI
setup-keypair:
	solana-keygen new --outfile ~/.config/solana/id.json
	solana config set --keypair ~/.config/solana/id.json

# Sync the program ID
sync-keys:
	anchor keys sync

# Build the program
build:
	anchor build

# Deploy the program to Solana
deploy:
	solana program deploy $(PROGRAM_SO)

# Clean the build artifacts
clean:
	cargo clean

# Update dependencies
update-deps:
	cargo update

# Run a local test validator
start-validator:
	solana-test-validator

# Stop the local test validator
stop-validator:
	pkill solana-test-validator

config-wallet-local:
	solana config set --url http://localhost:8899

config-wallet-dev:
	solana config set --url https://api.devnet.solana.com

# Full setup: install prerequisites, update dependencies, sync keys, and build
setup-dapp: setup-keypair sync-keys build 

.PHONY: setup-keypair sync-keys build deploy clean update-deps start-validator stop-validator
