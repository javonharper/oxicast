

watch:
	watchexec -w src make run
run: build
	./target/debug/oxicast --name Maple

build:
	cargo build
