# Description of the Python library, as automatically generated

```python
>>> import home.pytrace.pytrace as tr
>>> help(tr)
```
```
Help on module home.pytrace.pytrace in home.pytrace:

NAME
    home.pytrace.pytrace

CLASSES
    builtins.object
        builtins.Axes
        builtins.Camera
        builtins.Cfg
        builtins.Cone
        builtins.Cradle
        builtins.Cylinder
        builtins.Disc
        builtins.EmptyCone
        builtins.EmptyCylinder
        builtins.InfinitePlane
        builtins.Parallelogram
        builtins.RGB
        builtins.Rhomboid
        builtins.Sky
        builtins.Sphere
        builtins.Texture
        builtins.Triangle
        builtins.Vec

    class Axes(object)
     |  Axes(scale, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  build(self, /)
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  scale

    class Camera(object)
     |  Camera(x, y, z, /)
     |
     |  Methods defined here:
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  set_target(self, x, y, z, /)
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  angle
     |
     |  aperture
     |
     |  aspect
     |
     |  distance
     |
     |  rise
     |
     |  tilt

    class Cfg(object)
     |  Cfg(wth, hgt, iter, /)
     |
     |  Methods defined here:
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  add_obj(self, object, /)
     |
     |  clear(self, /)
     |
     |  end_movie(self, /)
     |
     |  frame(self, name, /)
     |
     |  populate(self, object, /)
     |
     |  render(self, name, /)
     |
     |  set_background(self, r, g, b, /)
     |
     |  set_cam(self, camera, /)
     |
     |  set_sky(self, sky, /)
     |
     |  silence(self, /)
     |
     |  start_movie(self, name, /)
     |
     |  true_background(self, /)
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  hgt
     |
     |  iter
     |
     |  nbsync
     |
     |  wth

    class Cone(object)
     |  Cone(vertex, direction, angle, begin, end, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class Cradle(object)
     |  Cradle(position, rotation, size)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  build(self, /)
     |
     |  raise_ball(self, amount, /)
     |
     |  set_time(self, t, /)
     |
     |  tick(self, dt, /)
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  position
     |
     |  rotation
     |
     |  size

    class Cylinder(object)
     |  Cylinder(center1, center2, radius, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class Disc(object)
     |  Disc(center, normal, radius, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class EmptyCone(object)
     |  EmptyCone(vertex, direction, angle, begin, end, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class EmptyCylinder(object)
     |  EmptyCylinder(center1, center2, radius, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class InfinitePlane(object)
     |  InfinitePlane(origin, normal, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class Parallelogram(object)
     |  Parallelogram(vertex, edge1, edge2, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class RGB(object)
     |  RGB(r, g, b, /)
     |
     |  Methods defined here:
     |
     |  __add__(self, value, /)
     |      Return self+value.
     |
     |  __iter__(self, /)
     |      Implement iter(self).
     |
     |  __mod__(self, value, /)
     |      Return self%value.
     |
     |  __radd__(self, value, /)
     |      Return value+self.
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __rmod__(self, value, /)
     |      Return value%self.
     |
     |  __rsub__(self, value, /)
     |      Return value-self.
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  __sub__(self, value, /)
     |      Return self-value.
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  black()
     |
     |  blue()
     |
     |  brown()
     |
     |  cyan()
     |
     |  dkblue()
     |
     |  dkgreen()
     |
     |  dkgrey()
     |
     |  dkred()
     |
     |  green()
     |
     |  grey()
     |
     |  ltblue()
     |
     |  ltgreen()
     |
     |  ltgrey()
     |
     |  ltred()
     |
     |  magenta()
     |
     |  orange()
     |
     |  purple()
     |
     |  red()
     |
     |  turquoise()
     |
     |  white()
     |
     |  yellow()

    class Rhomboid(object)
     |  Rhomboid(vertex, edge1, edge2, edge3, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class Sky(object)
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  uniform(r, g, b, /)

    class Sphere(object)
     |  Sphere(center, radius, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class Texture(object)
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  dielectric(color, index, /)
     |
     |  lambertian(color, /)
     |
     |  light(color, /)
     |
     |  metal(color, fuzzy, /)

    class Triangle(object)
     |  Triangle(vertex, edge1, edge2, texture, /)
     |
     |  Methods defined here:
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.

    class Vec(object)
     |  Vec(x, y, z)
     |
     |  Methods defined here:
     |
     |  __add__(self, value, /)
     |      Return self+value.
     |
     |  __iter__(self, /)
     |      Implement iter(self).
     |
     |  __mul__(self, value, /)
     |      Return self*value.
     |
     |  __neg__(self, /)
     |      -self
     |
     |  __radd__(self, value, /)
     |      Return value+self.
     |
     |  __repr__(self, /)
     |      Return repr(self).
     |
     |  __rmul__(self, value, /)
     |      Return value*self.
     |
     |  __rsub__(self, value, /)
     |      Return value-self.
     |
     |  __rtruediv__(self, value, /)
     |      Return value/self.
     |
     |  __str__(self, /)
     |      Return str(self).
     |
     |  __sub__(self, value, /)
     |      Return self-value.
     |
     |  __truediv__(self, value, /)
     |      Return self/value.
     |
     |  ----------------------------------------------------------------------
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  x
     |
     |  y
     |
     |  z

DATA
    __all__ = ['__doc__', 'Cfg', 'Camera', 'Vec', 'Sky', 'RGB', 'Texture',...

FILE
    home.pytrace.so
```
