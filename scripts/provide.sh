#!/usr/bin/env bash
echo uploading...
scp target/arm-unknown-linux-gnueabihf/debug/rpi-remote-control $1:/home/pi
echo done, executing...
ssh -t -t $1 ./rpi-remote-control