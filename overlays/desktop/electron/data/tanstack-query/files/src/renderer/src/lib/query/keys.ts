export const queryKeys = {
  system: ["system"] as const,
  systemVersion: ["system", "version"] as const,
  auth: {
    all: ["auth"] as const,
    me: ["auth", "me"] as const,
  },
};
