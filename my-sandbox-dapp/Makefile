# Install Node.js using nvm
install-node:
	curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
	export NVM_DIR="$${HOME}/.nvm" && \
	[ -s "$$NVM_DIR/nvm.sh" ] && \. "$$NVM_DIR/nvm.sh" && \
	[ -s "$$NVM_DIR/bash_completion" ] && \. "$$NVM_DIR/bash_completion" && \
	nvm install 22 && \
	nvm use 22 && \
	node -v && \
	npm -v

# Install Rust and Cargo using rustup
install-rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
	source $$HOME/.cargo/env && \
	rustc --version

# Install Solana CLI
install-solana:
	sh -c "$(curl -sSfL https://release.solana.com/v1.18.17/install)"
	zsh -c 'echo "export PATH=$$HOME/.local/share/solana/install/active_release/bin:$$PATH" >> ~/.zshrc && \
	source ~/.zshrc && \
	solana --version'

# Install anchor globally using Cargo
install-anchor:
	cargo install --git https://github.com/project-serum/anchor --tag v0.30.1 anchor-cli

# Install pnpm globally
install-pnpm:
	npm install -g pnpm

# Install all prerequisites
install: install-node install-solana install-rust install-anchor install-pnpm

# Full setup: install prerequisites, update dependencies, sync keys, and build
setup: install update-deps build

.PHONY: install-node install-solana install-yarn install-rust install-anchor install-pnpm setup-keypair sync-keys build deploy clean update-deps start-validator stop-validator install setup
