import { BrowserWindow, Notification, utilityProcess, type UtilityProcess } from "electron";
import PQueue from "p-queue";
import { randomUUID } from "node:crypto";
import { join } from "node:path";

import type { ActivityState } from "../../../shared/ipc/activity";
import { activityChannels } from "../../../shared/ipc/activity";
import { log } from "../../infra/logger";

const idle: ActivityState = {
  jobId: null,
  status: "idle",
  progress: 0,
  step: "",
  message: "",
  startedAt: null,
  finishedAt: null,
};

let state: ActivityState = idle;
let child: UtilityProcess | null = null;
const queue = new PQueue({ concurrency: 1 });

function publish(next: ActivityState): ActivityState {
  state = next;

  for (const window of BrowserWindow.getAllWindows()) {
    if (!window.isDestroyed()) {
      window.webContents.send(activityChannels.event, state);
    }
  }

  return state;
}

function workerPath(): string {
  return join(__dirname, "workers", "activity-worker.js");
}

function startJob(): Promise<ActivityState> {
  if (state.status === "running") {
    return Promise.reject(new Error("A job is already running."));
  }

  const jobId = randomUUID();
  publish({
    jobId,
    status: "running",
    progress: 0,
    step: "Starting worker",
    message: "",
    startedAt: new Date().toISOString(),
    finishedAt: null,
  });

  const worker = utilityProcess.fork(workerPath(), [], { serviceName: "activity-worker" });
  child = worker;

  void queue.add(async () => {
    await new Promise<void>((done) => {
      worker.on("message", (message: { type?: string; progress?: number; step?: string; message?: string }) => {
        if (message.type === "progress") {
          publish({
            ...state,
            progress: message.progress ?? state.progress,
            step: message.step ?? state.step,
          });
        }
        if (message.type === "complete") {
          publish({
            ...state,
            status: "complete",
            progress: 100,
            step: "Complete",
            finishedAt: new Date().toISOString(),
          });
          done();
        }
        if (message.type === "error") {
          publish({
            ...state,
            status: "failed",
            message: message.message ?? "Job failed.",
            finishedAt: new Date().toISOString(),
          });
          done();
        }
      });
      worker.on("exit", (code) => {
        if (state.status === "running") {
          publish({
            ...state,
            status: code === 0 ? "complete" : "failed",
            progress: code === 0 ? 100 : state.progress,
            finishedAt: new Date().toISOString(),
          });
        }
        done();
      });
    });
    if (Notification.isSupported() && !BrowserWindow.getAllWindows().some((window) => window.isFocused())) {
      new Notification({ title: "Activity complete", body: "The local import job finished." }).show();
    }
    log("info", "activity job finished", { jobId });
    if (child === worker) child = null;
  });

  return Promise.resolve(state);
}

export function getActivity(): ActivityState {
  return state;
}

export function cancelActivity(): ActivityState {
  if (state.status !== "running") return state;
  child?.postMessage("cancel");
  child?.kill();
  child = null;

  return publish({
    ...state,
    status: "cancelled",
    step: "Cancelled",
    finishedAt: new Date().toISOString(),
  });
}

export function retryActivity(): Promise<ActivityState> {
  if (state.status === "running") return Promise.reject(new Error("A job is already running."));

  return startJob();
}

export function startActivity(): Promise<ActivityState> {
  return startJob();
}
