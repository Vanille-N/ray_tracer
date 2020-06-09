# Creating a new sky texture

[![crates.io](http://meritbadge.herokuapp.com/pytrace)](https://crates.io/crates/pytrace)
[![API](https://docs.rs/pytrace/badge.svg)](https://docs.rs/pytrace)
[![crates.io](http://meritbadge.herokuapp.com/pytrace_core)](https://crates.io/crates/pytrace_core)
[![API](https://docs.rs/pytrace_core/badge.svg)](https://docs.rs/pytrace_core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Both `rstrace` and `pytrace` support adding a special (non-uniform) texture to the sky.

The only requirement is that said texture has to be provided **at runtime** as an image with format PPM (**P3 only**, refer to [the specification](http://netpbm.sourceforge.net/doc/ppm.html)).

By default, Gimp provides the correct format when asked to 'Export As...'.
