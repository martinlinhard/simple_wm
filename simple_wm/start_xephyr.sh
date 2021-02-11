#!/bin/bash
XEPHYR=$(whereis -b Xephyr | cut -f2 -d' ')

"$XEPHYR" \
    :100 \
    -ac \
    -screen 1280x720 #\
    #-host-cursor
