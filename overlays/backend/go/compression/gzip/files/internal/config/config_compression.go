package config

import "errors"

func (c Config) validateCompression() error {
	if c.QueueCompressionMinBytes < 0 {
		return errors.New("queue compression min bytes must not be negative")
	}

	if c.QueueCompressionMaxDecompressedBytes <= 0 {
		return errors.New("queue compression max decompressed bytes must be positive")
	}

	if c.QueueCompressionMaxDecompressedBytes < c.QueueCompressionMinBytes {
		return errors.New("queue compression max decompressed bytes must be at least the min bytes")
	}

	return nil
}
