/** @type {import("eslint").Linter.Config} */
const config = {
	parser: "@typescript-eslint/parser",
	parserOptions: {
		project: true,
	},
	plugins: ["@typescript-eslint"],
	extends: [
		"next/core-web-vitals",
		"plugin:@typescript-eslint/recommended-type-checked",
		"plugin:@typescript-eslint/stylistic-type-checked",
		"plugin:@tanstack/eslint-plugin-query/recommended",
	],
	rules: {
		"@typescript-eslint/no-unused-vars": ["warn", { argsIgnorePattern: "^_" }],
		"@typescript-eslint/no-misused-promises": "off",
	},
};

module.exports = config;
