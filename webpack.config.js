const path = require("path");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const fs = require('fs');

loadOutput = (environment) => {
    const filename = environment.production ? "scripts/[name].[hash].min.js" : "scripts/[name].js";
    const chunkFilename = environment.production ? "[name].[hash].min.js" : "[name].js";
    return {
        path: path.resolve(__dirname, "dist"),
        filename,
        chunkFilename,
        publicPath: "/"
    };
};

loadPlugins = (environment) => {

    let wasmPackArgs = "--no-typescript";
    if (environment.production) wasmPackArgs += " --release";

    return [
        new HtmlWebpackPlugin({
			template: "./front/index.html",
			favicon: "./front/image/iced.png",
            chunks: ["app"],
            hash: true
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, `./`),
            extraArgs: wasmPackArgs
        }),
        new CleanWebpackPlugin()
    ];
};

getImages = () => {
   const dir = "./front/image/";
   return fs.readdirSync(dir).map(x => x = dir + x);
}

module.exports = environment => {

    const entry = { 
        app: "./front/app.js",
        img: getImages()
    };
    const output = loadOutput(environment);
    const plugins = loadPlugins(environment);

    const module = {
        rules: [
        {
            test: /\.(png|jpe?g|gif)$/,
            use: [{
                loader: 'file-loader',
                options: {
                    name: '[path][name].[ext]',
                },
            }]
        }
    ]}

    //Base Config
    const config = { entry, output, plugins, module };

    //Additional Production Config
    if (environment.production) {
        config.devtool = "source-map";
    }

    //Additional Development Config
    else {
        config.devServer = {
            contentBase: path.join(__dirname, "dist"),
            open: true,
            overlay: { errors: true, warnings: false },
            port: 5000,
            watchContentBase: true,
            historyApiFallback: true
        };
    }

    return config;
};