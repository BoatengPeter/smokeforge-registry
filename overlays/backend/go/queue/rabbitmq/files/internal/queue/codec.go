package queue

// Codec serializes and deserializes job envelopes.
type Codec interface {
	Encode(job Job) ([]byte, error)
	Decode(data []byte) (Job, error)
}

// JSONCodec is the default job codec.
type JSONCodec struct{}

// Encode serializes a job as JSON.
func (JSONCodec) Encode(job Job) ([]byte, error) {
	return job.Encode()
}

// Decode parses a JSON job envelope.
func (JSONCodec) Decode(data []byte) (Job, error) {
	return Decode(data)
}
