# Ray Tracer

[![](https://img.shields.io/badge/github-Vanille--N/ray__tracer-8da0cb?logo=github)](https://github.com/Vanille-N/ray_tracer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`pytrace` [![crates.io](http://meritbadge.herokuapp.com/pytrace)](https://crates.io/crates/pytrace)
[![API](https://docs.rs/pytrace/badge.svg)](https://docs.rs/pytrace)

`pytrace_core` [![crates.io](http://meritbadge.herokuapp.com/pytrace_core)](https://crates.io/crates/pytrace_core)
[![API](https://docs.rs/pytrace_core/badge.svg)](https://docs.rs/pytrace_core)

## A Rust library for ray path tracing and image generation

<img src="img/NeV.gif" size=600>

### Contents
- `libtrace` contains most of the path tracing logic.
- `rstrace` is a standalone `.rs` file that gives access to the internal library. It can be used to generate scenes.
- `pytrace` provides an interface with Python, and creates a shared library that can be `import`ed.
- `data` groups a few images usable as sky textures
- `img` is a collection of images that either `rstrace` or `pytrace` are able to generate.
- `misc` contains a cheatsheet on how to properly orient molecule links.

For more informations, refer to the `README` in each subdirectories.


### Side note

An interesting corollary of having access to an accurate dielectic material and being able to create intersections is that I can simulate the behavior of a lens built from the intersection of two dielectric spheres.

Below is an animation that demonstrates how an object seen through a converging lens can be upside down. Remember that the ray tracer has no knowledge of what a lens is, all it does is calculate the path that individual light rays follow.

<img src="img/lens.gif" size=600>
