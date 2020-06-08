#!/usr/bin/python3

from pytrace import *
from os import system

tr = Cfg(400, 400, 200)
sky = Sky.uniform(RGB(1, 1, 1))
tr.set_sky(sky)
#tr.set_background(RGB(0, 0, 0))

cam = Camera(0, 0, 0)

glass = Texture.dielectric(RGB(0.9, 0.9, 0.9), 1.7)
e = 1.5
r = 50

s1 = Sphere(Vec(0, 0, e/2 - r), r, glass)
s2 = Sphere(Vec(0, 0, -e/2 + r), r, glass)

rtext = Texture.lambertian(RGB(0.1, 0.1, 0.1))

ring = Cylinder(Vec(0, 0, e/10), Vec(0, 0, -e/10), (e*r)**(0.5), rtext) \
    - Cylinder(Vec(0, 0, e), Vec(0, 0, -e), (e*r)**(0.5) * 0.9, rtext)

def target(y, z):
    orig = Vec(-2, y, z)
    bg = Texture.lambertian(RGB(0.5, 0.8, 0))
    fg = Texture.lambertian(RGB(0, 0.2, 1))
    triangle = Triangle(orig, Vec(60, 0, 0), Vec(0, 20, 0), bg)
    arrowrec = Parallelogram(orig + Vec(2, 1, 0.01), Vec(1, 0, 0), Vec(0, 3, 0), fg)
    arrowhead = Triangle(orig + Vec(2.5, 6, 0.01), Vec(-1, -2, 0), Vec(1, -2, 0), fg)
    return triangle | arrowrec | arrowhead

cam.distance = 100
tr.set_cam(cam)

cnt = 0

def lpad(l, n):
    return "0" * (l - len(str(n))) + str(n)

tr.start_movie("lens")


tr.add_obj(s1 & s2)
tr.add_obj(target(15, -10))
tr.add_obj(ring)
for i in range(50):
    cam.angle = 90 * (1 - i / 49)
    cam.distance = 20 + 80 * i / 49
    tr.set_cam(cam)
    tr.frame()

for i in range(20):
    tr.frame()

cam.angle = 0
cam.distance = 100
tr.set_cam(cam)

for i in range(60, -9, -1):
    tr.clear()
    tr.add_obj(s1 & s2)
    tr.add_obj(target(i/4, -10))
    tr.add_obj(ring)
    tr.frame()

for i in range(20):
    tr.frame()

for i in range(120):
    tr.clear()
    tr.add_obj(s1 & s2)
    tr.add_obj(target(-2, -10-i))
    tr.add_obj(ring)
    tr.frame()

tr.end_movie()
