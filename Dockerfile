FROM messense/rust-musl-cross:armv7-musleabihf-latest

# Install dependencies for Webkit2Gtk (for Tauri)
RUN apt-get update && apt-get install -y \
  pkg-config \
  libgtk-3-dev \
  libwebkit2gtk-4.0-dev \
  libsoup-2.4-dev \
  libjavascriptcoregtk-4.0-dev

ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig:/usr/share/pkgconfig
ENV PKG_CONFIG_SYSROOT_DIR=/
ENV PKG_CONFIG_ALLOW_CROSS=1
