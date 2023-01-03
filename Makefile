configure:
	./configure.sh
	
build:
	pushd frontend; npm run build; \
	cp public/* dist/assets/; popd; \
	pushd backend; cargo build

run:
	cd backend; \
	RUST_LOG=debug cargo run

dev:
	make build; RUST_LOG=debug make run

test:
	pushd backend; \
	RUST_BACKTRACE=1 RUST_LOG=debug cargo test

clean:
	pushd backend; cargo clean; popd; \
	pushd frontend; npm run clean

lint:
	pushd frontend; npm run lint; popd; \
	pushd backend; cargo fmt; cargo clippy
