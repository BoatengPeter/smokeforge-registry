package queue

import (
	"testing"
	"time"

	"github.com/google/uuid"
)

func TestJSONCodecRoundTrip(t *testing.T) {
	t.Parallel()

	job := Job{
		ID:        uuid.New(),
		Type:      "test.job",
		Payload:   []byte(`{"value":true}`),
		CreatedAt: time.Now().UTC(),
	}

	codec := JSONCodec{}

	encoded, err := codec.Encode(job)
	if err != nil {
		t.Fatalf("encode: %v", err)
	}

	decoded, err := codec.Decode(encoded)
	if err != nil {
		t.Fatalf("decode: %v", err)
	}

	if decoded.ID != job.ID || decoded.Type != job.Type {
		t.Fatalf("job = %+v, want %+v", decoded, job)
	}
}
