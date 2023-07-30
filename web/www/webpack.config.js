const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: './src/bootstrap.ts',
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: 'bootstrap.js',
  },
  mode: "development",
  module: {
    rules: [
      {
          test: /\.[jt]sx?$/,
          loader: 'esbuild-loader',
          options: {
            target: 'es2020'
          }
      },
    ]
  },
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
