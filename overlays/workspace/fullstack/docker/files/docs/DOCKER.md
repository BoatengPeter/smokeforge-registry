# Fullstack Docker

```bash
make docker-up
# or: docker compose up --build
```

The root `compose.yaml` extends the API and web compose files and adds the database, migration, and BFF wiring. `web` receives
`API_URL=http://api:8080` so the BFF reaches the API over the Compose network.
The browser still uses same-origin `/api/auth/*` routes on port 3000.

`docker compose config` validates the merged files.
