all: build_release copy_404

build_release: 
	@echo "Building release ๐ง"
	@trunk build 

copy_404:
	@echo "Copying 404.html ๐"
	@cp 404.html docs/

format: 
	@echo "Format ๐งน"
	@cargo fmt --all -- --check

lint: 
	@echo "Linting ๐งน"
	@cargo clippy --all -- -D warnings

pedantic:
	@echo "Linting (pedantic) ๐งน"
	@cargo clippy --all -- -D warnings -D clippy::pedantic

