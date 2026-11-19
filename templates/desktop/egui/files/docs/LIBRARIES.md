# Libraries

| Concern | Crate | Version |
| --- | --- | --- |
| UI | eframe / egui | 0.37 |
| Tables/images | egui_extras | 0.37 |
| Tiling | egui_tiles | 0.18 |
| Charts | egui_plot | 0.38 |
| Toasts | egui_notify | 0.24 |
| Native dialogs | rfd | 0.18 |
| DB | rusqlite / r2d2_sqlite / refinery | 0.40 / 0.35 / 0.10 |
| HTTP | reqwest / tokio | 0.13 / 1.54 |
| Secrets | keyring | 4.2 (`v1`) |
| i18n | rust-i18n | 4.2 |
| Testing | egui_kittest | 0.37 |
| Packaging | cargo-packager | 0.11 |

Rejected: `egui_dock`, `egui-phosphor`, and `egui-toast` because they target
egui 0.36 and would break the exact-version policy.
