

watch:
	watchexec -w src make run
run: build
	./target/debug/oxicast --dir /Users/javon/Documents/Media/Podcasts

build:
	cargo build
