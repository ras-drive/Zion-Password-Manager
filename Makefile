configure:
	./configure.sh
	
build:
	cd frontend; npm run build; \
	cp public/* dist/assets/; cd ..; \
	cd backend; cargo build

run:
	cd backend; \
	RUST_LOG=debug cargo run

docker_run:
	cd backend/migrations; \
	diesel migration run; \
	cd ../../backend; \
	RUST_LOG=debug ADDR=DOCKER cargo run

docker_test:
	cd backend/migrations; \
	diesel migration run; \
	cd ../../backend; \
	RUST_LOG=debug ADDR=DOCKER cargo test

dev:
	make build; RUST_LOG=debug make run

test:
	cd backend; \
	RUST_BACKTRACE=1 RUST_LOG=debug cargo test

clean:
	cd backend; cargo clean; cd ..; \
	cd frontend; npm run clean

lint:
	cd frontend; npm run lint; cd ..; \
	cd backend; cargo fmt; cargo clippy; cd ..; \
	shellcheck *.sh;
