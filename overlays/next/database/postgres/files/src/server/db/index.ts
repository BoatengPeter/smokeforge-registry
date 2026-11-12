import "server-only";
import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import * as schema from "@/server/db/schema";
import { serverEnv } from "@/server/env";

const client = postgres(serverEnv.DATABASE_URL, {
  max: 10,
  prepare: false,
});

export const db = drizzle(client, { schema });
