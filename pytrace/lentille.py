#!/usr/bin/python3

from pytrace import *
from os import system

tr = Cfg(100, 100, 10)
sky = Sky.uniform(RGB(1, 1, 1))
tr.set_sky(sky)
#tr.set_background(RGB(0, 0, 0))

cam = Camera(0, 0, 0)

glass = Texture.dielectric(RGB(0.9, 0.9, 0.9), 1.7)
e = 1.5
r = 50

s1 = Sphere(Vec3(0, 0, e/2 - r), r, glass)
s2 = Sphere(Vec3(0, 0, -e/2 + r), r, glass)

rtext = Texture.lambertian(RGB(0.1, 0.1, 0.1))

ring = Cylinder(Vec3(0, 0, e/2), Vec3(0, 0, -e/2), (e*r)**(0.5), rtext).diff(Cylinder(Vec3(0, 0, e), Vec3(0, 0, -e), (e*r)**(0.5) * 0.9, rtext))

target = lambda y, z: Triangle(Vec3(-2, y, z), Vec3(60, 0, 0), Vec3(0, 20, 0), Texture.lambertian(RGB(0.8, 0.4, 0))).union(Parallelogram(Vec3(0, y+1, z+1), Vec3(1, 0, 0), Vec3(0, 3, 0), Texture.lambertian(RGB(0.9, 0, 0)))).union(Triangle(Vec3(0.5, y+6, z+1), Vec3(-1, -2, 0), Vec3(1, -2, 0), Texture.lambertian(RGB(0.9, 0, 0))))

cam.distance = 100
tr.set_cam(cam)

cnt = 0

def lpad(l, n):
    return "0" * (l - len(str(n))) + str(n)

for i in range(60, -9, -1):
    tr.clear()
    tr.add_obj(s1.inter(s2))
    tr.add_obj(target(i/4, -10))
    tr.add_obj(ring)
    tr.render("lentille-" + lpad(5, cnt))
    cnt += 1
    print(cnt)
    #raise KeyboardInterrupt

for i in range(20):
    tr.render("lentille-" + lpad(5, cnt))
    cnt += 1
    print(cnt)

for i in range(120):
    tr.clear()
    tr.add_obj(s1.inter(s2))
    tr.add_obj(target(-2, -10-i))
    tr.add_obj(ring)
    tr.render("lentille-" + lpad(5, cnt))
    cnt += 1
    print(cnt)

system("rm lens.avi")
system("ffmpeg -pattern_type glob -framerate 25 -i \"img-lentille-*.ppm\" -vcodec libx264 lens.avi")
system("rm img-lentille-*.ppm")
