# Creating a new sky texture

Both `rstrace` and `pytrace` support adding a special (non-uniform) texture to the sky.

The only requirement is that said texture has to be provided **at runtime** as an image with format PPM (**P3 only**, refer to [the specification](http://netpbm.sourceforge.net/doc/ppm.html)).

By default, Gimp provides the correct format when asked to 'Export As...'.
