/* eslint-env node */
require("@rushstack/eslint-patch/modern-module-resolution");

module.exports = {
  root: true,
  extends: [
    "plugin:vue/vue3-strongly-recommended",
    "plugin:vue/vue3-recommended",
    "plugin:vue/vue3-essential",
    "eslint:recommended",
    "@vue/eslint-config-typescript",
    "prettier",
  ],
  overrides: [
    {
      files: ["cypress/e2e/**/*.spec.ts", "cypress/support/**/*.ts"],
      extends: ["plugin:cypress/recommended"],
    },
  ],
  rules: {
    "vue/multi-word-component-names": "off",
    "no-unused-vars": "off",
    "@typescript-eslint/no-unused-vars": [
      "warn",
      {
        argsIgnorePattern: "^_",
        varsIgnorePattern: "^_",
        caughtErrorsIgnorePattern: "^_",
      },
    ],
  },
  parserOptions: {
    ecmaVersion: "latest",
  },
};
