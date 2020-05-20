#!/usr/bin/python3

import pytrace as tr

trace = tr.Cfg(100, 100, 20)
trace.add_cam(tr.Camera(25, 25, 0))
trace.add_sky(tr.Sky.blank())
trace.add_obj()

trace.silence()
trace.render()
