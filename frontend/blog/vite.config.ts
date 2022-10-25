import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";
import { ConfigEnv, defineConfig } from "vite";
import compressPlugin from "vite-plugin-compression";
import { resolve } from "path";
// https://vitejs.dev/config/
export default ({ command, mode }: ConfigEnv) =>
    defineConfig({
        build: {
            outDir: "../../www/blog"
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
            port: 3000,
            proxy: {
                "^/api": {
                    target: "http://localhost:8080",
                    changeOrigin: true,
                },
            }
        }
    });
