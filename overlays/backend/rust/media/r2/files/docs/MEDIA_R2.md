# Cloudflare R2 media provider

R2 uses the same private S3 media API. Set `MEDIA_S3_ENDPOINT` and
`MEDIA_S3_PUBLIC_ENDPOINT` to `https://<account>.r2.cloudflarestorage.com`,
set `MEDIA_S3_REGION=auto`, and provide an R2 API token with object read/write
access. Original images only; transformations are not included in the Rust
provider.
