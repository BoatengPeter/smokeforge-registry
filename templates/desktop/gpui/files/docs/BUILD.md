# Build prerequisites

## Linux

Install the native development packages before building or testing:

```bash
sudo apt-get install -y \
  build-essential pkg-config \
  libfontconfig1-dev libfreetype6-dev \
  libxkbcommon-dev libxkbcommon-x11-dev \
  libwayland-dev libx11-xcb-dev libxcb1-dev
```

GPUI needs a GPU-capable session; test both Wayland and X11 on your target
distribution. The generated `.cargo/config.toml` enables fontconfig dlopen for
machines that have the runtime library but not the development package.

## macOS

Install the Xcode command-line tools and Rust. GPUI uses the native macOS
windowing and text stacks.

## Windows

Install the MSVC build tools and Rust. Use the `stable-x86_64-pc-windows-msvc`
toolchain.

## CI

CI installs the Linux development packages before `cargo check`, `cargo test`,
and packaging.
