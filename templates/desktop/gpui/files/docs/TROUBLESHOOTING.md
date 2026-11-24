# Troubleshooting

## `-lxkbcommon` / `-lxkbcommon-x11`

Install the Linux XKB development packages:

```bash
sudo apt-get install -y libxkbcommon-dev libxkbcommon-x11-dev
```

See [BUILD.md](BUILD.md) for Fedora, Arch, and the full dependency list.

## Blank or missing window on Linux

Confirm the session has a working Wayland or X11 compositor and a GPU driver.
Test both backends on the target machine.

## Keyring warning

The app continues with an in-memory token store. Install or unlock the OS
keyring/Secret Service for persistent refresh tokens.

## Sync conflicts

Open Account/Activity and choose **Keep local** or **Use server**. No conflict is
auto-resolved.

## Watch mode (`make dev`)

See [DEVELOPMENT.md](DEVELOPMENT.md#troubleshooting): install cargo-watch with
`make dev-tools`, and quit any other running copy of the app, because the
single-instance handoff makes the new process exit.
