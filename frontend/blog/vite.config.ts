import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";
import { ConfigEnv, defineConfig } from "vite";
import compressPlugin from "vite-plugin-compression";
import mkcert from "vite-plugin-mkcert";
import { VitePWA } from "vite-plugin-pwa";
import { resolve } from "path";
import fs from "fs";
// https://vitejs.dev/config/
export default ({ command, mode }: ConfigEnv) =>
    defineConfig({
        build: {
            outDir: "../../www/blog",
        },
        plugins: [
            vue(),
            vueJsx(),
            AutoImport({
                /* options */
                imports: ["vue", "vue-router", "pinia", "@vueuse/core"],
                dts: "./src/types/auto-imports.d.ts",
            }),
            Components({
                resolvers: [NaiveUiResolver()],
                directoryAsNamespace: true,
                extensions: ["vue"],
                dirs: ["src/components", "src/pages"],
                dts: "./src/types/components.d.ts",
            }),
            compressPlugin({
                verbose: true,
                disable: false,
                threshold: 10240,
                algorithm: "gzip",
                ext: ".gz",
            }),
            VitePWA({
                registerType: "autoUpdate",
                workbox: {
                    navigateFallbackDenylist: [/admin/i],
                    runtimeCaching: [
                        {
                            urlPattern: /api\/(.*?)/i, // 接口缓存 此处填你想缓存的接口正则匹配
                            handler: "CacheFirst",
                            options: {
                                cacheName: "interface-cache",
                            },
                        },
                        {
                            urlPattern: /(.*?)\.(js|css|ts)/, // js /css /ts静态资源缓存
                            handler: "CacheFirst",
                            options: {
                                cacheName: "js-css-cache",
                            },
                        },
                        {
                            urlPattern: /(.*?)\.(png|jpe?g|svg|gif|bmp|psd|tiff|tga|eps)/, // 图片缓存
                            handler: "CacheFirst",
                            options: {
                                cacheName: "image-cache",
                            },
                        },
                    ],
                },
                manifest: {
                    name: "ggtmBlog",
                    short_name: "blog",
                    description: "blog App",
                    theme_color: "#ffffff",
                    icons: [
                        {
                            src: "pwa-192x192.png",
                            sizes: "192x192",
                            type: "image/png",
                        },
                        {
                            src: "pwa-512x512.png",
                            sizes: "512x512",
                            type: "image/png",
                        },
                    ],
                },
            }),
            mkcert(),
        ],
        resolve: {
            alias: {
                "@": resolve(__dirname, "./src"),
            },
        },
        server: {
            https: true,
            port: 3000,
            proxy: {
                "^/api": {
                    changeOrigin: true,
                    target: {
                        protocol: "http:",
                        host: "localhost",
                        port: 8080,
                    },
                },
            },
        },
    });
