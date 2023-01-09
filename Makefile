all: clean dev cleanup

build: clean release move cleanup

dev:
	@echo "Running dev-mode"
	cargo tauri dev --verbose

release:
	@echo "Building release"
	cargo tauri build

move:
	@echo "Moving files"
	mv ./dist/*.wasm

cleanup:
	@echo "Cleaning up"
	rm -rf ./dist

clean:
	@echo "Cleaning"
	rm -rf ./dist ./target