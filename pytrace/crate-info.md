# PyTrace

[![](https://img.shields.io/badge/github-Vanille--N/ray__tracer-8da0cb?logo=github)](https://github.com/Vanille-N/ray_tracer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![API](https://docs.rs/pytrace/badge.svg)](https://docs.rs/pytrace)

### How to build

```shell
cargo build --release
cp target/release/libpytrace.so pytrace.so
```

That last step will only allow you to import the resulting library from your current directory. You may want to replace it with any of the following :
```shell
cp target/release/libpytrace.so usr/bin/pytrace.so
mv target/release/libpytrace.so target/release/pytrace.so; export PATH=$(pwd)/target/release:$PATH
...
```

You can now `import pytrace as tr` from any Python script, as long as `pytrace.so` is in your working directory or in your `PATH`.

The Github repository provides:
- `LIB.md` containing the autogenerated docs (available from a running Python instance with `help(pytrace)`)
- A few working examples in the form of executable `.py` files
- A sample of images and videos that were generated with the help of this library

### Note
It is possible to abort the process of creating an image by pressing `Ctrl + C`, but when doing so, the process will immediately `exit`. This makes little difference if you run a script, but it does if you run a REPL. Be warned that if you abort the `render` function, your REPL session will be terminated.
