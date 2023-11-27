const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = (env) => {
    return {
        entry: {
            main: './src/main.tsx',
        },
        resolve: {
            extensions: ['.ts', '.tsx', '.js', '.json'],
            alias: {
                '@component': path.resolve(__dirname, 'src/component'),
                '@pages': path.resolve(__dirname, 'src/pages'),
                '@layout': path.resolve(__dirname, 'src/layout'),
                '@assets': path.resolve(__dirname, 'src/assets'),
                '@model': path.resolve(__dirname, 'src/model'),
                '@services': path.resolve(__dirname, 'src/services'),
                '@utils': path.resolve(__dirname, 'src/utils'),
                '@hooks': path.resolve(__dirname, 'src/hooks')
            }
        },
        mode: 'development',
        devtool: 'source-map',
        output: {
            publicPath: '/',
            filename: '[name].js',
            chunkFilename: '[name].chunk.js'
        },
        devServer: {
            port: 9000,
            historyApiFallback: {
                disableDotRule: true
            },
            static: {
                directory: path.join(__dirname, 'public')
            },
        },
        optimization: {
            splitChunks: {
                chunks: 'all'
            }
        },
        module: {
            rules: [
                {
                    test: /\.tsx?|jsx$/,
                    use: {
                        loader: require.resolve('swc-loader'),
                        options: {
                            jsc: {
                                parser: {
                                    syntax: 'typescript',
                                    jsx: true
                                },
                                transform: {
                                    react: {
                                        runtime: 'automatic',
                                        pragma: 'React.createElement',
                                        pragmaFrag: 'React.Fragment',
                                        throwIfNamespace: true,
                                        development: false,
                                        useBuiltins: false
                                    }
                                }
                            }
                        }
                    }
                },
                {
                    test: /\.(css)$/,
                    use: [
                        'style-loader',
                        {
                            loader: 'css-loader',
                            options: {
                                sourceMap: true,
                                modules: {
                                    mode: (resourcePath) => {
                                        if (/pure.css$/i.test(resourcePath)) {
                                            return 'pure';
                                        }

                                        if (/global.css$/i.test(resourcePath)) {
                                            return 'global';
                                        }

                                        return 'local';
                                    },
                                    localIdentName: env.production
                                        ? '[contenthash]'
                                        : '[path][name]__[local]--[hash:base64:5]',
                                    exportLocalsConvention: 'camelCaseOnly'
                                }
                            }
                        },
                        'postcss-loader'
                    ]
                },
                {
                    test: /\.(jpg|png|gif|ico|icns)$/,
                    loader: 'file-loader',
                    options: {
                        name: '[hash:10].[ext]'
                    }
                },
                // 使用 @svgr/webpack loader 导入 svg 作为组件使用
                {
                    test: /\.svg$/i,
                    issuer: /\.[jt]sx?$/,
                    use: [
                        '@svgr/webpack',
                        {
                            loader: 'file-loader',
                            options: {
                                name: '[hash:10].[ext]'
                            }
                        }
                    ]
                }
            ]
        },
        externals: [],
        plugins: [
            new HtmlWebpackPlugin({
                template: './src/index.html',
                filename: 'index.html',
                publicPath: '/',
                chunks: ['main', 'vendor']
            })
        ]
    };
};
