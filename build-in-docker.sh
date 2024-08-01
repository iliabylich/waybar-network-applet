#!/usr/bin/env bash

cargo build --release
dh binary

cp /build/waybar-network-applet_$BUILD_VERSION\_amd64.deb /shared
cp /build/waybar-network-applet-$BUILD_VERSION/target/release/waybar-network-applet /shared
