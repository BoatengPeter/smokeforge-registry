"use client";

import { useEffect } from "react";
import posthog from "posthog-js";

let initialized = false;

export function PostHogBootstrap() {
  useEffect(() => {
    const key = process.env.NEXT_PUBLIC_POSTHOG_KEY;

    if (!key || initialized) {
      return;
    }

    posthog.init(key, {
      api_host:
        process.env.NEXT_PUBLIC_POSTHOG_HOST ?? "https://us.i.posthog.com",
      autocapture: false,
      capture_pageview: "history_change",
      disable_session_recording: true,
      person_profiles: "identified_only",
      persistence: "memory",
      opt_out_capturing_by_default: true,
    });
    initialized = true;
  }, []);

  return null;
}
