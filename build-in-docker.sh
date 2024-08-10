#!/usr/bin/env bash

set -eux

dh binary

cp /build/waybar-network-applet_${BUILD_VERSION}_amd64.deb /shared
cp /build/waybar-network-applet-$BUILD_VERSION/waybar-network-applet /shared
