FROM debian:unstable
ARG BUILD_VERSION

RUN apt update

RUN apt install -y debhelper pkg-config
RUN apt install -y libglib2.0-dev libpango1.0-dev libgdk-pixbuf-2.0-dev libgtk-4-dev libgtk4-layer-shell-dev

COPY *.c /build/waybar-network-applet-$BUILD_VERSION/
COPY *.h /build/waybar-network-applet-$BUILD_VERSION/
COPY Makefile /build/waybar-network-applet-$BUILD_VERSION/
COPY debian /build/waybar-network-applet-$BUILD_VERSION/debian

WORKDIR /build/waybar-network-applet-$BUILD_VERSION

RUN sed -i "s/VERSION_TEMPLATE/$BUILD_VERSION/g" debian/changelog
RUN sed -i "s/VERSION_TEMPLATE/$BUILD_VERSION/g" debian/control


ENV BUILD_VERSION="$BUILD_VERSION"

CMD ["/shared/build-in-docker.sh"]
