/** @type {import("prettier").Config} */
const config = {
	semi: true,
	singleQuote: false,
	trailingComma: "all",
	printWidth: 100,
	useTabs: true,
	plugins: [require.resolve("prettier-plugin-tailwindcss")],
};

module.exports = config;
