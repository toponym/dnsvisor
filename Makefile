.PHONY: coverage coverage-clean

coverage: coverage-clean
	export RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="coverage-%p-%m.profraw"; \
	cargo test --lib
	grcov . -s src --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

coverage-clean:
	- rm *.profraw
	- rm -r ./target/debug/coverage
