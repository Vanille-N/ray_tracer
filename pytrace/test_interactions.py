#!/usr/bin/python3

from pytrace import *

tr = Cfg(100, 100, 20)
tr.set_sky(Sky("../data/sky.ppm"))
cam = Camera(0, 0, 0)
cam.distance = 8
tr.set_cam(cam)

a = Sphere(Vec(0, 0, 0), 1, Texture.lambertian(RGB(1, 0, 0)))
b = Sphere(Vec(1, 0, 0), 1, Texture.lambertian(RGB(0, 1, 0)))
c = Sphere(Vec(0.5, 0.5, 0), 1, Texture.lambertian(RGB(0, 0, 1)))

tr.add_obj(a - b - c)

tr.render("union")
