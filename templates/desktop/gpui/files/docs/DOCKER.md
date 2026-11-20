# Docker

Docker can build and run the GPUI app, but a GUI application still needs the
host display server and GPU access. The container image includes the native
runtime libraries; the host provides the display socket.

```mermaid
flowchart LR
    HostDisplay[X11/Wayland socket] --> Container[GPUI container]
    HostGPU[/dev/dri] --> Container
    Container --> Window[Native window]
```

## Build

```bash
docker build -t gpui-app .
```

The build stage installs the `-dev` packages. The runtime stage installs the
versioned runtime libraries, so the final image does not contain the compiler
or development headers.

## Run with X11

```bash
xhost +local:docker
docker run --rm -it \
  -e DISPLAY="$DISPLAY" \
  -v /tmp/.X11-unix:/tmp/.X11-unix:rw \
  --device /dev/dri \
  --group-add video \
  gpui-app
xhost -local:docker
```

## Run with Compose

```bash
xhost +local:docker
docker compose run --rm app
xhost -local:docker
```

## Wayland

Expose the Wayland socket and runtime directory:

```bash
docker run --rm -it \
  -e WAYLAND_DISPLAY="$WAYLAND_DISPLAY" \
  -e XDG_RUNTIME_DIR=/tmp \
  -v "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:/tmp/$WAYLAND_DISPLAY" \
  --device /dev/dri \
  --group-add video \
  gpui-app
```

If the window does not appear, test X11 first. GPU/Vulkan drivers and the
compositor must be available on the host.
