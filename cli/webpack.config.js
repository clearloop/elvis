const path = require("path");
const webpack = require('webpack');
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

module.exports = {
  devServer: {
    compress: true,
    contentBase: "./dist",
    hot: true,
    host: 'localhost',
    open: true,
    port: 3000,
    watchContentBase: true,
    writeToDisk: true,
  },
  devtool: "inline-source-map",
  entry: [
    "./src/index",
     "webpack/hot/only-dev-server",
  ],
  mode: "development",
  module: {
    rules: [
      {
        exclude: /node_modules/,
        test: /\.tsx?$/,
        use: "ts-loader",
      },
    ],
  },
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "dist"),
    publicPath: "/",
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      title: "Output Management",
    }),
    new webpack.HotModuleReplacementPlugin(),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "../lib/"),
    },
    extensions: [ ".tsx", ".ts", ".js" ],
  },
};
