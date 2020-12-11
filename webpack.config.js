const path = require("path");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { wasm, experiments } = require("webpack");

module.exports = {
  entry: "./www-src/index.tsx",
  mode: "development",
  experiments: {
    syncWebAssembly: true,
    topLevelAwait: true
  },
  module: {
    rules: [
      {
        test: /\.(ts|tsx)$/,
        exclude: /(node_modules|bower_components)/,
        use: ["babel-loader", "ts-loader"]
      },
      {
        test: /\.(css)$/,
        exclude: /(node_modules|bower_components)/,
        use: ["style-loader", "css-loader"]
      }
    ]
  },
  resolve: { extensions: [".tsx", ".ts", ".js"]},
  output: {
    path: path.resolve(__dirname, "dist/"),
    publicPath: "/dist/",
    filename: "bundle.js"
  },
  devServer: {
    contentBase: path.join(__dirname, "public/"),
    port: 3000,
    publicPath: "http://localhost:3000/dist/",
    hotOnly: true
  },
  plugins: [
    new webpack.HotModuleReplacementPlugin(),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outDir: "perlin",
      outName: "perlin",
    }),
  ],
};