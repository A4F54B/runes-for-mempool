const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: {
    background: './background.js',
    index: './index.js'
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js',
    webassemblyModuleFilename: 'runes.wasm',
    publicPath: '/',
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  devtool: 'cheap-module-source-map',
  mode: process.env.NODE_ENV || 'development',
  experiments: {
    asyncWebAssembly: true
  }
};
