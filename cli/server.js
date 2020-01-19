const webpackDevServer = require('webpack-dev-server');
const webpack = require('webpack');

const config = require('./webpack.config.js');
const compiler = webpack(config);
const server = new webpackDevServer(compiler);

server.listen(5000, 'localhost', () => {
  console.log('dev server listening on port 5000');
});
