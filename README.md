# Ray Tracer

## A Rust library for ray path tracing and image generation

### Contents
- `libtrace` contains most of the path tracing logic.
- `rstrace` is a standalone `.rs` file that gives access to the internal library. It can be used to generate scenes.
- `pytrace` provides an interface with Python, and creates a shared library that can be `import`ed.
- `data` groups a few images usable as sky textures
- `img` is a collection of images that either `rstrace` or `pytrace` are able to generate.
- `misc` contains a cheatsheet on how to properly orient molecule links.

For more informations, refer to the `README` in each subdirectories.
