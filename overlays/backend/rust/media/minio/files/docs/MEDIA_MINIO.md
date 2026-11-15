# MinIO media provider

Start MinIO, imgproxy, and nginx with the locally available images:

```bash
make media-up
```

| Service | URL |
| --- | --- |
| MinIO API | `http://localhost:9000` |
| MinIO console | `http://localhost:9001` |
| imgproxy via nginx | `http://localhost:8081` |

The API uses `MEDIA_S3_ENDPOINT=http://minio:9000` in Docker and
`MEDIA_S3_PUBLIC_ENDPOINT=http://localhost:9000` for browser presigned URLs.
`MEDIA_IMGPROXY_BASE_URL` points at nginx, which caches imgproxy responses.
`MEDIA_IMGPROXY_KEY` and `MEDIA_IMGPROXY_SALT` are hex-encoded and must match in
the API and imgproxy. Requests can use `width`, `height`, `fit`, `format`, and
`quality`; transformations are signed and expire with `MEDIA_DOWNLOAD_TTL_SECONDS`.

Configure bucket CORS for browser `PUT` uploads.
