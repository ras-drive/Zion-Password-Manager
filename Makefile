build:
	cd server; cargo build --release
run:
	cd server; \
	RUST_BACKTRACE=1 cargo run

test:
	cd server; \
	RUST_BACKTRACE=1 cargo test

clean:
	cd server; \
	cargo clean