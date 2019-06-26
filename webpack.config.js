const webpack = require("webpack");
const path = require("path");
const package = require(path.resolve(__dirname, "package.json"));

const CopyWebpackPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./js/index.js",
  output: {
    path: dist,
    filename: "bundle.js"
  },
  devServer: {
    contentBase: dist
  },
  module: {
    rules: [
      {
        test: /\.(png|svg|jpg|gif)$/,
        use: ["file-loader"]
      }
    ]
  },
  plugins: [
    new CopyWebpackPlugin([
      {
        from: path.resolve(__dirname, "public"),
        to: path.resolve(__dirname, "dist")
      }
    ]),

    new webpack.DefinePlugin({
      PUBLIC_URL: JSON.stringify(withoutTrailingSlash(package.homepage))
    }),

    new HtmlWebpackPlugin({
      template: "index.html"
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "crate")
      // WasmPackPlugin defaults to compiling in "dev" profile. To change that, use forceMode: 'release':
      // forceMode: 'release'
    })
  ]
};

function withoutTrailingSlash(url) {
  if (typeof url !== "string") {
    throw new TypeError("URL must be string.");
  } else if (url.slice(-1) === "/") {
    return url.slice(0, -1);
  } else {
    return url;
  }
}
