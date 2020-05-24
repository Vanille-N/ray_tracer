#!/usr/bin/python3

################################
# THIS FILE IS OUT OF DATE     #
# It has not yet been modified #
# to adjust to the new version #
# of the library               #
################################

from pytrace import *
from os import system

def lpad(l, n):
    return "0" * (l - len(str(n))) + str(n)

tr = Cfg(200, 200, 20)
tr.add_sky(Sky("data/sky.ppm"))
tr.populate()
tr.silence()

cam = Camera(0, .5, 0)
cam.set_distance(5)
cam.set_rise(30)

for i in range(180):
    cam.set_angle(i*2)
    tr.add_cam(cam)
    tr.render(lpad(5, i))
    print(i)


system("rm sky.avi")
system("ffmpeg -pattern_type glob -framerate 25 -i \"img-*.ppm\" -vcodec libx264 sky.avi")
system("rm img-*.ppm")
