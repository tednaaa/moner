import { defineConfig } from "cypress";

export default defineConfig({
  e2e: {
    baseUrl: "http://localhost",

    supportFile: "cypress/src/support/e2e.ts",
    specPattern: "cypress/src/e2e/**/*.cy.ts",

    fixturesFolder: "cypress/src/fixtures",
    videosFolder: "cypress/assets/videos",
    screenshotsFolder: "cypress/assets/screenshots",
    downloadsFolder: "cypress/assets/downloads",
  },
});
