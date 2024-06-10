import { defineConfig } from "cypress";

export default defineConfig({
  e2e: {
    specPattern: "cypress/e2e/**/*.spec.ts",
    baseUrl: "http://localhost:4173",
  },
});
