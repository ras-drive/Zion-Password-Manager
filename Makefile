configure:
	./configure.sh
	
build:
	cd frontend; npm run build; \
	cp public/* dist/assets/; cd ../; \
	cd backend; cargo build

run:
	cd backend; \
	RUST_BACKTRACE=1 cargo run

dev:
	make build; make run

test:
	cd backend; \
	RUST_BACKTRACE=1 cargo test

clean:
	cd backend; cargo clean; cd ..;\
	cd frontend; npm run clean
