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
        builtins.Camera
        builtins.Cfg
        builtins.Cone
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
        builtins.Vec3

    class Camera(object)
     |  Camera(x, y, z, /)
     |
     |  Methods defined here:
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
     |  add_obj(self, object, /)
     |
     |  populate(self, /)
     |
     |  render(self, name, /)
     |
     |  set_background(self, color, /)
     |
     |  set_cam(self, sky, /)
     |
     |  set_sky(self, sky, /)
     |
     |  silence(self, /)
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
     |  wth

    class Cone(object)
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
     |  begin
     |
     |  dir
     |
     |  end
     |
     |  orig
     |
     |  texture

    class Cylinder(object)
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  center1
     |
     |  center2
     |
     |  radius
     |
     |  texture

    class Disc(object)
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  center
     |
     |  normal
     |
     |  radius
     |
     |  texture

    class EmptyCone(object)
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
     |  begin
     |
     |  dir
     |
     |  end
     |
     |  orig
     |
     |  texture

    class EmptyCylinder(object)
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  center1
     |
     |  center2
     |
     |  radius
     |
     |  texture

    class InfinitePlane(object)
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  normal
     |
     |  orig
     |
     |  texture

    class Parallelogram(object)
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  a
     |
     |  texture
     |
     |  u
     |
     |  v

    class RGB(object)
     |  RGB(r, g, b, /)
     |
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
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  a
     |
     |  texture
     |
     |  u
     |
     |  v
     |
     |  w

    class Sky(object)
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  uniform()

    class Sphere(object)
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  center
     |
     |  radius
     |
     |  texture

    class Texture(object)
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
     |  Static methods defined here:
     |
     |  __new__(*args, **kwargs) from builtins.type
     |      Create and return a new object.  See help(type) for accurate signature.
     |
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |
     |  a
     |
     |  texture
     |
     |  u
     |
     |  v

    class Vec3(object)
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
    __all__ = ['__doc__', 'Cfg', 'Camera', 'Vec3', 'Sky', 'RGB', 'Texture'...

FILE
    home.pytrace.so
```
