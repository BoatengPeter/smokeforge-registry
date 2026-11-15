# Queue compression

The gzip codec compresses queue envelopes at or above
`QUEUE_COMPRESSION_MIN_BYTES` and detects gzip on decode. Jobs below the
threshold remain plain JSON. Invalid gzip or JSON is rejected before a handler
runs.
