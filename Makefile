build:
	cd kernel && cargo build --release

run: build
	./scripts/runner.sh

clean:
	cd kernel && cargo clean
