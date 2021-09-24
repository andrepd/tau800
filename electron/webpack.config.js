const path = require('path');

module.exports = {
  entry: './src/live/live.js',
  output: {
    path: path.resolve(__dirname, 'src/bundled'),
    filename: 'live.bundle.js',
  },
  mode: 'development',
};