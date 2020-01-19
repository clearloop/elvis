import webpack from "webpack";
import webpackDevServer from "webpack-dev-server";
import config from "./webpack.config";

const compiler = webpack(config);
const server = new webpackDevServer(compiler);

server.listen(5000, "localhost", () => {
  console.log("dev server listening on port 5000");
});
