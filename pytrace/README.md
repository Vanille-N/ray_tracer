# PyTrace

### How to use

- Run `cargo build --release`
- Copy `target/release/libpytrace.so` anywhere in your `$PATH` under the name `pytrace.so` (for example `cp target/release/libpytrace.so usr/bin/pytrace.so` or `cp target/release/libpytrace.so pytrace.so` or `mv target/release/libpytrace.so target/release/pytace.so ; export PATH=$(pwd)/target/release:$PATH`
- You can now `import` a ray tracer using `import pytrace as tr`

See `LIB.md` to see the autogenerated docs.
A few `.py` files provide working examples.