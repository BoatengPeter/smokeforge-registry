# Libraries

| Concern | Crate | Version |
| --- | --- | --- |
| UI | gpui-kit | 0.10.0 |
| GPUI snapshot | gpui-pre | 0.3.12 (exact via gpui-kit) |
| DB | rusqlite / refinery | 0.40 / 0.10 |
| HTTP | reqwest | 0.13 |
| Secrets | keyring | 4.2 |
| i18n | rust-i18n | 4.2 |
| Packaging | cargo-packager | 0.11 |

Do not mix GPUI snapshots. Do not add raw Zed Git dependencies while GPUI Kit supplies the pinned snapshot.
