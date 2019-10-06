const {join} = require('path');

module.exports = {
  entry: "./src/index.js",
  output: {
    path: join(__dirname, "dist"),
    filename: "app.js",
  },
  mode: "development",

  devServer:  {
    contentBase: 'dist'    
  }
}
