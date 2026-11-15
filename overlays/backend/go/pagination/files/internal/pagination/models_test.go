package pagination

import "testing"

const paginationFixtureSecret = "smokeforge-test-pagination-secret-32"

func TestOffsetMetadata(t *testing.T) {
	t.Parallel()

	meta := OffsetMetadata(Offset{Page: 2, PageSize: 20}, 45)
	if meta.Mode != "offset" || meta.Offset != 20 || meta.HasMore != true {
		t.Fatalf("meta = %#v", meta)
	}

	if meta.TotalPages == nil || *meta.TotalPages != 3 {
		t.Fatalf("total pages = %#v", meta.TotalPages)
	}

	last := OffsetMetadata(Offset{Page: 3, PageSize: 20}, 45)
	if last.HasMore {
		t.Fatalf("last page has more: %#v", last)
	}
}

func TestCursorMetadata(t *testing.T) {
	t.Parallel()

	more := CursorMetadata(10, "next")
	if more.Mode != "cursor" || more.Limit != 10 || !more.HasMore || more.NextCursor != "next" {
		t.Fatalf("meta = %#v", more)
	}

	end := CursorMetadata(10, "")
	if end.HasMore {
		t.Fatalf("end has more: %#v", end)
	}
}

func TestNewPage(t *testing.T) {
	t.Parallel()

	page := NewPage[string](nil, CursorMetadata(10, ""))
	if page.Items == nil {
		t.Fatal("items must be initialized")
	}
}
