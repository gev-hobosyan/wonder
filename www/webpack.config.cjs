const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");
const { experiments } = require("webpack");

module.exports = {
	entry: "./bootstrap.js",
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "bootstrap.js",
	},
	mode: "development",
	plugins: [
		new CopyWebpackPlugin({
			patterns: [{ from: "index.html", to: "index.html" }],
		}),
	],
	experiments: {
		asyncWebAssembly: true,
	},
	module: {
		rules: [
			{
				test: /\.wasm$/,
				type: "webassembly/async",
			},
		],
	},
};
