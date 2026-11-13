import type { Config } from "jest";
import nextJest from "next/jest.js";

const createJestConfig = nextJest({ dir: "./" });

const config: Config = {
  clearMocks: true,
  testEnvironment: "jsdom",
  setupFilesAfterEnv: ["<rootDir>/jest.setup.ts"],
  moduleNameMapper: {
    "^@/(.*)$": "<rootDir>/src/$1",
    "^server-only$": "<rootDir>/src/test/server-only.ts",
  },
  modulePathIgnorePatterns: ["<rootDir>/.next/", "<rootDir>/.smokeforge/"],
  testPathIgnorePatterns: [
    "<rootDir>/.next/",
    "<rootDir>/e2e/",
    "<rootDir>/node_modules/",
    "<rootDir>/.agents/",
    "<rootDir>/.claude/",
    "<rootDir>/.impeccable/",
  ],
};

export default createJestConfig(config);
