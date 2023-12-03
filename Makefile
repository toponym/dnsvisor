.PHONY: coverage coverage-clean

coverage: coverage-clean
	export RUSTFLAGS="-Cinstrument-coverage"
	cargo build
	export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"
	cargo test
	grcov . -s src --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

coverage-clean:
	- rm *.profraw
	- rm -r ./target/debug/coverage
