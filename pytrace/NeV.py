#!/usr/bin/python3

from pytrace import *

tr = Cfg(1000, 500, 20)

cam = Camera(75, 15, 0)
cam.angle = 20
cam.rise = 20
cam.distance = 300
cam.aperture = 20

tr.set_cam(cam)

sky = Sky.uniform(RGB(1, 1, 1))
tr.set_sky(sky)
tr.set_background(RGB(0, 0, 0))

t1 = Texture.lambertian(RGB(0.0, 0.7, 0.7));
t2 = Texture.lambertian(RGB(0.2, 0.8, 0.3));
t3 = Texture.lambertian(RGB(0.5, 0.7, 0.0));

x = Vec3(10, 0, 0);
y = Vec3(0, 10, 0);
z = Vec3(0, 0, 30);

nbase = Rhomboid(x.mul(-0.2), y.mul(0.5), x.mul(1.4), z, t1)
nlbar = Rhomboid(x.mul(0.25), y.mul(4.55), x.mul(0.5), z, t1)

nlserif = Rhomboid(y.mul(0.5), y.mul(0.25), x, z, t1).diff(Cylinder(y.mul(0.75).add(z.mul(-0.1)), y.mul(0.75).add(z.mul(1.1)), 2.5, t1)).diff(Cylinder(x.add(y.mul(0.75)).add(z.mul(-0.1)), x.add(y.mul(0.75)).add(z.mul(1.1)), 2.5, t1))

nmidbar = Rhomboid(x.mul(-0.2).add(y.mul(5.)), x.mul(1.65), x.mul(5.).add(y.mul(-5.)), z, t1).diff(Rhomboid(x.mul(0.62).add(y.mul(4.7)).add(z.mul(-0.1)), x.mul(0.6), x.mul(5.).add(y.mul(-5.)), z.mul(1.2), t1)).diff(Cylinder(x.mul(5.), x.mul(7.), 40., t1))

nrbar = Rhomboid(x.mul(4.8), y.mul(5.), x.mul(0.5), z, t1)
ntop = Rhomboid(x.mul(4.30).add(y.mul(4.5)), y.mul(0.5), x.mul(1.4), z, t1)

nrserif = Rhomboid(x.mul(4.55).add(y.mul(4.25)), y.mul(0.25), x, z, t1).diff(Cylinder(x.mul(4.55).add(y.mul(4.25)).add(z.mul(-0.1)), x.mul(4.55).add(y.mul(4.25)).add(z.mul(1.1)), 2.5, t1)).diff(Cylinder(x.mul(5.55).add(y.mul(4.25)).add(z.mul(-0.1)), x.mul(5.55).add(y.mul(4.25)).add(z.mul(1.1)), 2.5, t1))

ecirc = Cylinder(x.mul(7.5).add(y.mul(1.5)), x.mul(7.5).add(y.mul(1.5)).add(z), 15., t2).diff(Cylinder(x.mul(7.5).add(y.mul(1.5)).add(z.mul(-0.1)), x.mul(7.5).add(y.mul(1.5)).add(z.mul(1.1)), 10., t2)).diff(Rhomboid(x.mul(7.5).add(y.mul(1.5)).add(z.mul(-0.1)), x.mul(10.).add(y.mul(5.)), x.mul(10.).add(y.mul(-5.)), z.mul(1.2), t2,))

ehbar = Rhomboid(x.mul(6.5).add(y.mul(1.)), x.mul(2.).add(y.mul(1.)), y.mul(0.57), z, t2)
vlbar = Rhomboid(x.mul(9.5), y.mul(5.), x.mul(0.5), z, t3)
vltop = Rhomboid(x.mul(9.).add(y.mul(4.5)), y.mul(0.5), x.mul(1.4), z, t3)

vlserif = Rhomboid(x.mul(9.25).add(y.mul(4.25)), y.mul(0.25), x, z, t3).diff(Cylinder(x.mul(9.25).add(y.mul(4.25)).add(z.mul(-0.1)), x.mul(9.25).add(y.mul(4.25)).add(z.mul(1.1)), 2.5, t3)).diff(Cylinder(x.mul(10.25).add(y.mul(4.25)).add(z.mul(-0.1)), x.mul(10.25).add(y.mul(4.25)).add(z.mul(1.1)), 2.5, t3))

vmidlo = Cylinder(x.mul(5.13).add(y.mul(9.88)), x.mul(5.13).add(y.mul(9.88)).add(z), 110.3, t3).inter(Rhomboid(x.mul(9.5), y.mul(5.), x.mul(6.), z.mul(1.2), t3)).diff(Cylinder(x.mul(5.13).add(y.mul(9.88)).add(z.mul(-0.1)), x.mul(5.13).add(y.mul(9.88)).add(z.mul(1.1)), 105., t3))

vmidhi = Cylinder(x.mul(5.13).add(y.mul(9.88)), x.mul(5.13).add(y.mul(9.88)).add(z), 100., t3).inter(Rhomboid(x.mul(9.5), y.mul(5.), x.mul(6.), z.mul(1.2), t3)).diff(Cylinder(x.mul(5.13).add(y.mul(9.88)).add(z.mul(-0.1)), x.mul(5.13).add(y.mul(9.88)).add(z.mul(1.1)), 95., t3))

vrtop = Rhomboid(x.mul(13.5).add(y.mul(4.5)), y.mul(0.5), x.mul(0.95), z, t3)

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

tr.render("NeV")
