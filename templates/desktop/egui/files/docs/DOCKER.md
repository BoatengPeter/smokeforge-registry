# Docker

Docker can build and run the egui app, but a GUI application still needs the
host display server and GPU access. The image includes egui/eframe runtime
libraries; the host provides the display socket.

```mermaid
flowchart LR
    HostDisplay[X11/Wayland socket] --> Container[egui container]
    HostGPU[/dev/dri] --> Container
    Container --> Window[Native window]
```

## Build

```bash
docker build -t smokeforge-egui .
```

## Run with X11

```bash
xhost +local:docker
docker run --rm -it \
  -e DISPLAY="$DISPLAY" \
  -v /tmp/.X11-unix:/tmp/.X11-unix:rw \
  --device /dev/dri \
  --group-add video \
  smokeforge-egui
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
  smokeforge-egui
```

The host compositor, GPU driver, and `/dev/dri` must be available. Test X11
first when debugging.
