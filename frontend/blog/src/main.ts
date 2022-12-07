import { createApp } from 'vue'
import { createPinia } from "pinia";
import App from './App.vue'
import { createHead } from "@vueuse/head";
import { router } from "./routers";
import { useRegisterSW } from 'virtual:pwa-register/vue'

useRegisterSW()

const head = createHead();
const pinia = createPinia();

const app = createApp(App)

app.use(head).use(router).use(pinia).mount("#app");



