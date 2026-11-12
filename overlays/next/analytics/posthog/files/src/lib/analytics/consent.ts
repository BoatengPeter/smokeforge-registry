"use client";

import posthog from "posthog-js";

export function optInAnalytics(): void {
  posthog.opt_in_capturing();
}

export function optOutAnalytics(): void {
  posthog.opt_out_capturing();
}

export function isAnalyticsEnabled(): boolean {
  return posthog.has_opted_in_capturing();
}
