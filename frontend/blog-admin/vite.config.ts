import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";
import { ConfigEnv, defineConfig } from "vite";
import compressPlugin from "vite-plugin-compression";
import path, { resolve } from "path";
import { loadEnv } from "vite";
import fs from "fs";

// https://vitejs.dev/config/
export default ({ command, mode }: ConfigEnv) =>
    defineConfig({
        base: loadEnv(mode, process.cwd()).VITE_ADMIN_URL,
        build: {
            outDir: "../../www/blog-admin",
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
        ],
        resolve: {
            alias: {
                "@": resolve(__dirname, "./src"),
            },
        },
        server: {
            port: 4000,
            base: "/admin",
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
