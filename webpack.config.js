const path = require('path');
const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const dotenv = require('dotenv').config({path: __dirname + '/.env'});

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
      new webpack.DefinePlugin({
        'process.env.baseurl': JSON.stringify(dotenv.parsed.baseurl),
        'process.env.gtmId': JSON.stringify(dotenv.parsed.gtmId)
      }),
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