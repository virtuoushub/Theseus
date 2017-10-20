all: build

build: 
	@mkdir -p ../$(USERSPACE_BUILD_DIR)
	@RUST_TARGET_PATH=../../cfg xargo build --target x86_64-theseus-user

clean:
	cargo clean
