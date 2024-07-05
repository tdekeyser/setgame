build: build-webapp build-server

run: build
	./target/release/server

build-webapp:
	cd webapp && pnpm run build

build-server:
	cargo build --release
