init:
	cargo install --path .
	pnpm install

diesel-instal:
	curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

dev:
	(trap "kill 0" SIGINT; cargo watch -x run & pnpm run dev:build:css)

migrate-up:
	diesel migration run

# revert migration
migrate-down:
	diesel migration revert -n 1
