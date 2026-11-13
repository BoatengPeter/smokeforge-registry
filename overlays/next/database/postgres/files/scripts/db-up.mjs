import { spawnSync } from "node:child_process";

const installHint =
  "Install Docker Desktop or Docker Engine: https://docs.docker.com/get-docker/";
const existingPostgresHint =
  "Or set DATABASE_URL in .env to an existing Postgres, then run: make db-migrate";

const version = spawnSync("docker", ["--version"], { stdio: "ignore" });
if (version.error || version.status !== 0) {
  console.error(`Docker is not installed.\n${installHint}\n${existingPostgresHint}`);
  process.exit(1);
}

const info = spawnSync("docker", ["info"], { stdio: "ignore" });
if (info.error || info.status !== 0) {
  console.error(
    "Docker is installed but the daemon is not running.\nStart Docker Desktop (macOS/Windows) or the Docker service (Linux), then retry.",
  );
  process.exit(1);
}

const up = spawnSync(
  "docker",
  ["compose", "-f", "compose.db.yaml", "up", "-d", "--wait", "db"],
  { stdio: "inherit" },
);
if (up.error || up.status !== 0) {
  console.error(
    "Could not start Postgres with Docker Compose.\nEnsure Docker Compose v2 is available (docker compose version).\nIf port 5432 is already in use, change the port in compose.db.yaml and DATABASE_URL in .env, then retry.",
  );
  process.exit(up.status ?? 1);
}

console.log("Postgres is ready. Run: make db-migrate");
