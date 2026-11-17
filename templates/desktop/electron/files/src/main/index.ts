import { app } from "electron";

import { bootstrap } from "./bootstrap";

void bootstrap().catch((error: unknown) => {
  console.error("desktop bootstrap failed", error);
  app.quit();
});
