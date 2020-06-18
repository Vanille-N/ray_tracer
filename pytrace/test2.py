#!/usr/bin/python3

from pytrace import *

tr = Cfg(300, 300, 20)
tr.set_sky(Sky("data/spectrum.ppm"))

cam = Camera(0, 0, 0)
cam.rise = 20
cam.distance = 5
tr.set_cam(cam)

ground = Texture.lambertian(RGB(1, 0, 0))
tr.add_obj(InfinitePlane(Vec(0, 0, 0), Vec(0, 1, 0), ground))

s = Sphere(Vec(0, 0, 0), 1, Texture.metal(RGB(1, 1, 1), 0))
tr.add_obj(s)

tr.render("test2.ppm")
