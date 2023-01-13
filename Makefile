all: dev cleanup

build: release move cleanup

dev:
	@echo "Running dev-mode"
	cargo tauri dev

release:
	@echo "Building release"
	cargo tauri build

move:
	@echo "Moving files"
	mv ./dist/*.wasm ./target/

cleanup:
	@echo "Cleaning up"
	rm -rf ./dist
