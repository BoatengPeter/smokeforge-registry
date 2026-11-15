# MinIO media provider

`make media-up` starts MinIO, imgproxy, and nginx with the locally available images:

- `minio/minio:latest` stores private objects.
- `minio/mc:latest` creates the `media` bucket and leaves anonymous access disabled.
- `darthsim/imgproxy:v3.30.1` reads from the bucket and signs transform URLs.
- `nginx:1.28.2-alpine` caches transformed responses at `http://localhost:8081`.

The API uses `http://minio:9000` in Docker and `http://localhost:9000` when the API runs locally. The browser-facing presigned host is `http://localhost:9000`.

## Endpoints

| Service | URL |
| --- | --- |
| MinIO S3 | `http://localhost:9000` |
| MinIO console | `http://localhost:9001` |
| imgproxy via nginx | `http://localhost:8081` |

## Signing

`MEDIA_IMGPROXY_KEY` and `MEDIA_IMGPROXY_SALT` are hex-encoded and must match in the API and imgproxy. The builder signs the path with HMAC-SHA256 and URL-safe Base64, as documented by imgproxy. Transform URLs include an `expires` option. imgproxy only serves the private `media` bucket.

## Browser upload CORS

Configure the `media` bucket CORS for the frontend origin:

- Allowed methods: `PUT`
- Allowed headers: `Content-Type`
- Exposed headers: `ETag`

Use the MinIO console or `mc cors set` with a JSON CORS file.

## Commands

```bash
make media-up
make media-logs
make media-shell
make media-down
```
