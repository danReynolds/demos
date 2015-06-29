all: target/debug/libincrement.so
	ruby src/main.rb

release: target/release/libincrement.so
	ruby src/main.rb

target/release/libincrement.so: src/lib.rs Cargo.toml
	cargo build --release

target/debug/libincrement.so: src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
