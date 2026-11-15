# Cloudflare R2 media provider

R2 stores objects in a private bucket and returns short-lived presigned reads. The API uses the same S3 adapter as MinIO; no additional containers are required.

## Configuration

Set these values in `.env`:

```dotenv
MEDIA_S3_ENDPOINT=https://<account-id>.r2.cloudflarestorage.com
MEDIA_S3_PUBLIC_ENDPOINT=https://<account-id>.r2.cloudflarestorage.com
MEDIA_S3_REGION=auto
MEDIA_S3_BUCKET=<bucket-name>
MEDIA_S3_ACCESS_KEY=<r2-access-key-id>
MEDIA_S3_SECRET_KEY=<r2-secret-access-key>
MEDIA_S3_FORCE_PATH_STYLE=true
```

Create an R2 API token with object read and write access for the bucket. Keep the bucket private.

## CORS

R2 must allow direct browser uploads:

- Allowed origins: the frontend origin
- Allowed methods: `PUT`
- Allowed headers: `Content-Type`
- Exposed headers: `ETag`

## Transformations

R2 serves original images only. `GET /api/v1/media/{id}/url` with resize or format parameters returns `501 transformations_not_configured`. Use Cloudflare Images or another image service when transformations are required.

## Operations

Presigned PUT URLs expire after `MEDIA_UPLOAD_URL_TTL`; presigned reads expire after `MEDIA_DOWNLOAD_URL_TTL`. Complete the upload within the upload TTL or create a new reservation.
