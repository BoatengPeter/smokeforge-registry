# UI fonts

The app embeds Inter so text looks the same on every platform.

| File | Source | SHA-256 |
| --- | --- | --- |
| `Inter-Regular.ttf` | Inter 4.1, `extras/ttf/` from https://github.com/rsms/inter/releases/tag/v4.1 | `40d692fce188e4471e2b3cba937be967878f631ad3ebbbdcd587687c7ebe0c82` |
| `Inter-SemiBold.ttf` | same release | `78a843fade9d4612a5567302fb595b56976eb5fcebf4fea5a5912d638bafcde3` |

Licensed under the SIL Open Font License 1.1 (`OFL.txt`). Ship `OFL.txt` with
any distribution that includes the font files.

To change the UI font, replace these files and update the font names in the
theme module; keep the regular and semibold weights.
