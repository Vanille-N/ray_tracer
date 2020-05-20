#!/usr/bin/python3

import pytrace as tr

trace = tr.Cfg(100, 100, 20)
trace.make_cam()
trace.make_sky()
trace.add_obj()
trace.render()
