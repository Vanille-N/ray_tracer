# Ray Tracer

## A Rust library for ray path tracing and image generation

This repository bundles together three crates:
- `libtrace` contains most of the path tracing logic.
- `rstrace` is a standalone `.rs` file that gives access to the internal library. It can be used to generate scenes.
- `pytrace` provides an interface with Python, and creates a shared library that can be `import`ed.

### How to use

Refer to the three `README` in each subcrate.
