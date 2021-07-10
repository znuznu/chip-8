const { merge } = require('webpack-merge');
const common = require('./common.js');

let config = merge(common, {
    mode: 'production',
});

module.exports = config;