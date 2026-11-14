package config

import "testing"

func TestValidateCompression(t *testing.T) {
	t.Parallel()

	if err := (Config{QueueCompressionMinBytes: 1024, QueueCompressionMaxDecompressedBytes: 4096}).validateCompression(); err != nil {
		t.Fatalf("valid config: %v", err)
	}

	if err := (Config{QueueCompressionMinBytes: -1, QueueCompressionMaxDecompressedBytes: 4096}).validateCompression(); err == nil {
		t.Fatal("expected negative min error")
	}

	if err := (Config{QueueCompressionMinBytes: 1024, QueueCompressionMaxDecompressedBytes: 0}).validateCompression(); err == nil {
		t.Fatal("expected zero max error")
	}

	if err := (Config{QueueCompressionMinBytes: 4096, QueueCompressionMaxDecompressedBytes: 1024}).validateCompression(); err == nil {
		t.Fatal("expected max below min error")
	}
}
