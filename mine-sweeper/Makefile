dev:
	yarn tauri dev

test:
	@cd src-tauri && cargo test

fix:
	@yarn lint
	@yarn format
	@cd src-tauri && cargo +nightly fmt
	@cd src-tauri && cargo fix --allow-dirty --allow-staged
	@cd src-tauri && cargo clippy --fix --allow-dirty --allow-staged
	@cd src-tauri && cargo test

