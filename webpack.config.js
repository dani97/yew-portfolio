const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000,
      historyApiFallback: {
        index: './index.html'
      }
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "portfolio.js",
      webassemblyModuleFilename: "portfolio.wasm"
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            'style-loader',
            'css-loader',
            'sass-loader',
          ],
        },
        {
            test: /\.jpe?g$|\.gif$|\.png$/i,
            loader: "file-loader?name=[name].[ext]"
        }
      ],
    },
    plugins: [
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new HtmlWebpackPlugin({
        template: "index.html",
        filename: distPath + "/index.html"
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};