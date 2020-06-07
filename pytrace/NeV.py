#!/usr/bin/python3

from pytrace import *
from os import system

tr = Cfg(200, 100, 20)

cam = Camera(75, 15, 15)
cam.rise = 20
cam.distance = 300
cam.aperture = 20

sky = Sky.uniform(RGB(1, 1, 1))
tr.set_sky(sky)
tr.set_background(RGB(0, 0, 0))

t1 = Texture.lambertian(RGB(0.0, 0.7, 0.7));
t2 = Texture.lambertian(RGB(0.2, 0.8, 0.3));
t3 = Texture.lambertian(RGB(0.5, 0.7, 0.0));

x = Vec3(10, 0, 0);
y = Vec3(0, 10, 0);
z = Vec3(0, 0, 30);

nbase = Rhomboid(-x*0.2, y*0.5, x*1.4, z, t1)
nlbar = Rhomboid(x*0.25, y*4.55, x*0.5, z, t1)

nlserif = Rhomboid(y*0.5, y*0.25, x, z, t1) \
    - Cylinder(y*0.75 - z*0.1, y*0.75 + z*1.1, 2.5, t1) \
    - Cylinder(x + y*0.75 - z*0.1, x + y*0.75 + z*1.1, 2.5, t1)

nmidbar = Rhomboid(-x*0.2 + y*5., x*1.65, x*5 - y*5, z, t1) \
    - Rhomboid(x*0.62 + y*4.7 - z*0.1, x*0.6, x*5 - y*5, z*1.2, t1) \
    - Cylinder(x*5, x*7, 40, t1)

nrbar = Rhomboid(x*4.8, y*5, x*0.5, z, t1)
ntop = Rhomboid(x*4.30 + y*4.5, y*0.5, x*1.4, z, t1)

nrserif = Rhomboid(x*4.55 + y*4.25, y*0.25, x, z, t1) \
    - Cylinder(x*4.55 + y*4.25 - z*0.1, x*4.55 + y*4.25 + z*1.1, 2.5, t1) \
    - Cylinder(x*5.55 + y*4.25 - z*0.1, x*5.55 + y*4.25 + z*1.1, 2.5, t1)

ecirc = Cylinder(x*7.5 + y*1.5, x*7.5 + y*1.5 + z, 15., t2) \
    - Cylinder(x*7.5 + y*1.5 - z*0.1, x*7.5 + y*1.5 + z*1.1, 10., t2) \
    - Rhomboid(x*7.5 + y*1.5 - z*0.1, x*10. + y*5., x*10.- y*5., z*1.2, t2)

ehbar = Rhomboid(x*6.5 + y*1., x*2. + y*1., y*0.57, z, t2)
vlbar = Rhomboid(x*9.5, y*5., x*0.5, z, t3)
vltop = Rhomboid(x*9. + y*4.5, y*0.5, x*1.4, z, t3)

vlserif = Rhomboid(x*9.25 + y*4.25, y*0.25, x, z, t3) \
    - Cylinder(x*9.25 + y*4.25 - z*0.1, x*9.25 + y*4.25 + z*1.1, 2.5, t3) \
    - Cylinder(x*10.25 + y*4.25 - z*0.1, x*10.25 + y*4.25 + z*1.1, 2.5, t3)

vmidlo = Cylinder(x*5.13 + y*9.88, x*5.13 + y*9.88 + z, 110.3, t3) \
    & Rhomboid(x*9.5 - z*0.1, y*5., x*6., z*1.2, t3) \
    - Cylinder(x*5.13 + y*9.88 - z*0.1, x*5.13 + y*9.88 + z*1.1, 105., t3)

vmidhi = Cylinder(x*5.13 + y*9.88, x*5.13 + y*9.88 + z, 100., t3) \
    & Rhomboid(x*9.5 - z*0.1, y*5., x*6., z*1.2, t3) \
    - Cylinder(x*5.13 + y*9.88 - z*0.1, x*5.13 + y*9.88 + z*1.1, 95., t3)

vrtop = Rhomboid(x*13.5 + y*4.5, y*0.5, x*0.95, z, t3)

tr.add_obj(nbase);
tr.add_obj(nlbar);
tr.add_obj(nlserif);
tr.add_obj(nmidbar);
tr.add_obj(nrbar);
tr.add_obj(ntop);
tr.add_obj(nrserif);
tr.add_obj(ecirc);
tr.add_obj(ehbar);
tr.add_obj(vlbar);
tr.add_obj(vlserif);
tr.add_obj(vltop);
tr.add_obj(vmidlo);
tr.add_obj(vmidhi);
tr.add_obj(vrtop);

def lpad(l, n):
    return "0" * (l - len(str(n))) + str(n)

for i in range(180):
    cam.angle = i*2
    tr.set_cam(cam)
    tr.render("NeV-" + lpad(5, i))
    print(i)

system("rm sky.avi")
system("ffmpeg -pattern_type glob -framerate 25 -i \"img-NeV-*.ppm\" -vcodec libx264 sky.avi")
system("rm img-NeV-*.ppm")
