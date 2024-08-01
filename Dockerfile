FROM debian:unstable
ARG BUILD_VERSION

RUN apt update

RUN apt install -y debhelper pkg-config
RUN apt install -y rustc libglib2.0-dev libpango1.0-dev libgdk-pixbuf-2.0-dev libgtk-4-dev libgtk4-layer-shell-dev

RUN mkdir -p /build/waybar-network-applet-$BUILD_VERSION
COPY src /build/waybar-network-applet-$BUILD_VERSION/src
COPY Cargo.toml /build/waybar-network-applet-$BUILD_VERSION/Cargo.toml
COPY Cargo.lock /build/waybar-network-applet-$BUILD_VERSION/Cargo.lock
COPY debian /build/waybar-network-applet-$BUILD_VERSION/debian

WORKDIR /build/waybar-network-applet-$BUILD_VERSION

RUN sed -i "s/VERSION_TEMPLATE/$BUILD_VERSION/g" debian/changelog
RUN sed -i "s/VERSION_TEMPLATE/$BUILD_VERSION/g" debian/control


ENV BUILD_VERSION="$BUILD_VERSION"

CMD ["/shared/build-in-docker.sh"]
