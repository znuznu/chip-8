const { merge } = require('webpack-merge');
const common = require('./common.js');

let config = merge(common, {
    mode: 'development',
    devtool: 'eval-source-map',
});

module.exports = config;
