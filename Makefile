all: build-release

build-release:
	(cd ./extension && cargo build --release)
	cp ./extension/target/release/libryaml.dylib ./ryaml.so

build:
	(cd ./extension && cargo build)
	cp ./extension/target/debug/libryaml.dylib ./ryaml.so

test: build
	py.test -xv tests.py

clean:
	(cd ./extension && cargo clean)
	rm ./ryaml.so
