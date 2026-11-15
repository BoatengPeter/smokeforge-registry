# MinIO media provider

Start MinIO and create the private `media` bucket with:

```bash
make media-up
```

The API uses `MEDIA_S3_ENDPOINT=http://minio:9000` in Docker and
`MEDIA_S3_PUBLIC_ENDPOINT=http://localhost:9000` for browser presigned URLs.
Configure bucket CORS for browser `PUT` uploads.
