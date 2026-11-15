# Pagination

`internal/pagination` provides one contract for cursor and offset pagination.
It is opt-in: list handlers call the parser that matches their query and return
the generated metadata.

## Limits and configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `PAGINATION_CURSOR_SECRET` | generated secret | HMAC key for signed cursors |
| `PAGINATION_DEFAULT_LIMIT` | `20` | Default `page_size` or cursor `limit` |
| `PAGINATION_MAX_LIMIT` | `100` | Maximum page size |
| `PAGINATION_MAX_OFFSET` | `10000` | Deep-offset guard |

The secret must be at least 32 bytes. Never expose it to clients or logs.

Build limits once at the boundary:

```go
limits := pagination.Limits{
	DefaultLimit: cfg.PaginationDefaultLimit,
	MaxLimit:     cfg.PaginationMaxLimit,
	MaxOffset:    cfg.PaginationMaxOffset,
}
```

## Offset pagination

Use page/limit pagination for admin tables and bounded datasets where arbitrary
page jumps matter.

```go
offset, err := pagination.ParseOffset(c.Request.URL.Query(), limits)
if err != nil {
	// return the app's RFC 9457 validation response
}

rows, total := listItems(ctx, offset.PageSize, offset.OffsetValue())
c.JSON(http.StatusOK, pagination.NewPage(rows, pagination.OffsetMetadata(offset, total)))
```

The parser defaults `page` to 1 and `page_size` to the configured default,
rejects non-positive values, caps the page size, and rejects offsets beyond
`PAGINATION_MAX_OFFSET`.

Use a deterministic order and a tie-breaker:

```sql
SELECT id, title, created_at
FROM items
ORDER BY created_at DESC, id DESC
LIMIT $1 OFFSET $2;
```

## Cursor pagination

Use signed cursors for feeds, timelines, search-like lists, and tables over
large or changing data. Keyset pagination avoids deep `OFFSET` scans.

```go
codec, err := pagination.NewCodec(cfg.PaginationCursorSecret)
if err != nil {
	// configuration error
}

cursor, err := pagination.ParseCursor(c.Request.URL.Query(), limits)
if err != nil {
	// return validation
}

var values map[string]string
if cursor.Raw != "" {
	values, err = codec.Decode(cursor.Raw)
	if err != nil {
		// return validation
	}
}

rows, next := listItemsAfter(ctx, values["created_at"], values["id"], cursor.Limit)
nextCursor := ""
if next != nil {
	nextCursor, err = codec.Encode(map[string]string{
		"created_at": next.CreatedAt.Format(time.RFC3339Nano),
		"id":         next.ID,
	})
}

c.JSON(http.StatusOK, pagination.NewPage(
	rows,
	pagination.CursorMetadata(cursor.Limit, nextCursor),
))
```

Typical keyset query:

```sql
SELECT id, title, created_at
FROM items
WHERE (created_at, id) < ($1, $2)
ORDER BY created_at DESC, id DESC
LIMIT $3;
```

Always order by `(sort_key, id)` so equal timestamps cannot duplicate or skip
rows. Use the same tuple in the cursor. Cursors are opaque and HMAC-signed; a
tampered cursor fails before the query runs.

## Response shape

```json
{
  "items": [],
  "pagination": {
    "mode": "cursor",
    "limit": 20,
    "next_cursor": "opaque-signed-value",
    "has_more": true
  }
}
```

Offset pages also include `page`, `page_size`, `offset`, `total_items`, and
`total_pages`. Cursor pages intentionally do not compute totals; counting every
feed is expensive and often stale.

## Rules

- Do not accept client-controlled sort SQL; map sort choices to allow-listed
  columns.
- Do not use unbounded `LIMIT`; clamp every query.
- Do not deep-paginate with offset; switch to cursors for large feeds.
- Keep total counts optional; they are not part of cursor metadata.
- Return empty arrays, not `null`.
- Treat cursors as opaque; clients must not construct or inspect them.
