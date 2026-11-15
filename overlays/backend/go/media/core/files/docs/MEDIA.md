# Image media

Authenticated image uploads use a reservation flow:

1. `POST /api/v1/media/uploads` creates a pending row and returns a presigned `PUT` URL.
2. The client uploads the bytes directly to object storage with the returned headers.
3. `POST /api/v1/media/{id}/complete` checks the object's size and content type, then marks it ready.
4. `GET /api/v1/media/{id}/url` returns a short-lived private read URL.

All routes require a Bearer access token. Rows are scoped by `owner_id`; another user's media is reported as not found.

## Endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| `POST` | `/api/v1/media/uploads` | Reserve an upload |
| `POST` | `/api/v1/media/{id}/complete` | Verify and finish an upload |
| `GET` | `/api/v1/media/{id}` | Read metadata |
| `GET` | `/api/v1/media/{id}/url` | Read original or transformed URL |
| `DELETE` | `/api/v1/media/{id}` | Delete metadata and object |

`GET /url` accepts `width`, `height`, `fit` (`cover`, `contain`, `fill`), `format` (`jpeg`, `png`, `webp`), and `quality` (`1`-`100`).

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `MEDIA_S3_ENDPOINT` | `http://localhost:9000` | API-to-storage endpoint |
| `MEDIA_S3_PUBLIC_ENDPOINT` | `http://localhost:9000` | Browser-facing presigned host |
| `MEDIA_S3_REGION` | `us-east-1` | Bucket region |
| `MEDIA_S3_BUCKET` | `media` | Private bucket |
| `MEDIA_S3_ACCESS_KEY` | `minioadmin` | S3 access key |
| `MEDIA_S3_SECRET_KEY` | `minioadmin` | S3 secret key |
| `MEDIA_S3_FORCE_PATH_STYLE` | `true` | MinIO path style |
| `MEDIA_MAX_BYTES` | `10485760` | Maximum upload size |
| `MEDIA_ALLOWED_CONTENT_TYPES` | `image/jpeg,image/png,image/webp,image/gif` | Allowed image types |
| `MEDIA_UPLOAD_URL_TTL` | `15m` | Upload URL lifetime |
| `MEDIA_DOWNLOAD_URL_TTL` | `1h` | Read URL lifetime |

## CORS

Direct browser uploads need bucket CORS. Allow the application origin, `PUT`, and `Content-Type`. The API does not proxy upload bytes.

## Providers

- MinIO + imgproxy + nginx: see `docs/MEDIA_MINIO.md`.
- Cloudflare R2: see `docs/MEDIA_R2.md`; transformations are not available.
