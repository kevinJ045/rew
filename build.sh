#!/bin/sh
opath=./bin/ui
if [ $1 ]; then
  opath=$1
fi
g++ ./cpp/ui.cpp -o $opath `pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0 libwebsockets jsoncpp`
