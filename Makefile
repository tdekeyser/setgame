build: build-webapp build-server

run: build
	./target/release/api

build-webapp:
	cd webapp && pnpm run build

build-server:
	cargo build --release
