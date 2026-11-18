type ParentPort = {
  postMessage: (message: unknown) => void;
  on: (event: "message", listener: (event: { data: unknown }) => void) => void;
};

const parentPort = (process as NodeJS.Process & { parentPort?: ParentPort }).parentPort;
const steps = ["Reading source", "Validating rows", "Writing records", "Rebuilding search", "Finishing"];

let cancelled = false;

parentPort?.on("message", (event) => {
  if (event.data === "cancel") cancelled = true;
});

async function run(): Promise<void> {
  for (let index = 0; index < steps.length; index += 1) {
    if (cancelled) return;
    await new Promise((resolve) => setTimeout(resolve, 280));
    if (cancelled) return;
    parentPort?.postMessage({
      type: "progress",
      progress: Math.round(((index + 1) / steps.length) * 100),
      step: steps[index],
    });
  }

  parentPort?.postMessage({ type: "complete" });
}

void run().catch((error: unknown) => {
  parentPort?.postMessage({ type: "error", message: error instanceof Error ? error.message : "Job failed." });
});
