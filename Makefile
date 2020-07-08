all: build run

check:
	cargo check --release --target=x86_64-unknown-linux-musl


build:
	@mkdir -p ./dist
	@cargo build --release --target=x86_64-unknown-linux-musl
	@cp -v ./target/x86_64-unknown-linux-musl/release/prj01 ./dist
	@#ls -lh ./dist/
	@strip ./dist/prj01
	@#ls -lh ./dist/


run:
	@./dist/prj01
