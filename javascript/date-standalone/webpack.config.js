const path = require('path');
module.exports = {
  mode: 'production',
  target: 'es2019',
  devtool: 'cheap-module-source-map',
  optimization: {
    sideEffects: true
  },
  module: {
  },
  resolve: {
    extensions: [ '.ts', '.js' ],
  },
  output: {
    libraryTarget: "umd",
    globalObject: 'this',
    filename: 'index.js',
    path: path.join(__dirname, "build"),
    library: 'Shopify',
    chunkFormat: 'array-push'
  },
};

