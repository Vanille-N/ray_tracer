#!/usr/bin/python3

from pytrace import *
from os import system


tr = Cfg(600, 600, 50)
sky = Sky.uniform(RGB(1, 1, 1))
tr.set_sky(sky)
tr.set_background(RGB(0, 0, 0))

cam = Camera(0, 5, 0)
cam.distance = 45
cam.rise = 30

tr.start_movie("cradle")

crad = Cradle(Vec3(-5, 0, -5), 0, 10)
crad.raise_ball(60)
for i in range(360):
    crad.set_time(15 * i / 360)
    tr.clear()
    tr.populate(crad.build())
    cam.angle = -i
    tr.set_cam(cam)
    tr.frame()

tr.end_movie()
