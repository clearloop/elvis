import path from "path";
import webpack from "webpack";
import HtmlWebpackPlugin from "html-webpack-plugin";
import { CleanWebpackPlugin } from "clean-webpack-plugin";

const config: webpack.Configuration = {
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
  entry: path.resolve(__dirname, "./src/index.ts"),
  mode: "development",
  module: {
    rules: [{
      exclude: /node_modules/,
      test: /\.tsx?$/,
      use: "ts-loader",
    }, {
      enforce: "pre",
      test: /\.ts$/,
      use: [
        {
          loader: "tslint-loader",
          options: {
            tsConfigFile: "../tsconfig.json",
          },
        },
      ],
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

export default config;
