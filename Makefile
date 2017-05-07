ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread
endif

all: target/minirle_rust
	target/minirle_rust

target:
	mkdir -p $@

target/debug/libminirle_rust.a: src/lib.rs Cargo.toml
	cargo build

target/minirle_rust: target/test.o target/debug/libminirle_rust.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/test.o: src/test.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
