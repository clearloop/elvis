const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

module.exports = {
  devServer: {
    compress: true,
    contentBase: "./dist",
    hot: true,
    open: true,
    port: 3000,
    watchContentBase: true,
    writeToDisk: true,
  },
  devtool: "inline-source-map",
  entry: {
    app: path.join(__dirname, "./src/index.ts"),
  },
  mode: "development",
  module: {
    rules: [{
      exclude: /node_modules/,
      test: /\.tsx?$/,
      use: "ts-loader",
    },{
      test: /\.ts$/,
      enforce: 'pre',
      use: [
        {
          loader: "tslint-loader",
          options: {
	    tsConfigFile: "../tsconfig.json",
	  },
        }
      ]
    }],
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
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "../lib/"),
    },
    extensions: [ ".tsx", ".ts", ".js" ],
  },
};
