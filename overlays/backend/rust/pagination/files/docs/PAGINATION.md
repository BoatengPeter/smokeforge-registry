# Pagination

Use `internal::pagination` for signed cursors and bounded offset pagination.

- `parse_offset(page, page_size, limits)` rejects non-positive values, caps page
  size, and guards deep offsets.
- `parse_cursor(limit, cursor, limits)` keeps cursors opaque.
- `Codec` signs cursor maps with HMAC-SHA256.
- `Offset::metadata` and `cursor_metadata` produce the response metadata.
- `Page<T>` serializes `{ items, pagination }`.

Use `(created_at, id)` keyset queries for cursor feeds and `LIMIT/OFFSET` only
for bounded admin tables.
