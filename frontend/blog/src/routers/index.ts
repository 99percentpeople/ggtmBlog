import { createRouter, createWebHistory } from "vue-router";

const routes = [
    { path: "/", name: "home", component: () => import("@/pages/Home.vue") },
    { path: "/blog/:id(\\d+)?", name: "blog", component: () => import("@/pages/Blog.vue") },
    {
        path: "/search",
        name: "search",
        component: () => import("@/pages/Search.vue"),
    },
    { path: "/archive", name: "archive", component: () => import("@/pages/Archive.vue") },
    { path: "/about", name: "about", component: () => import("@/pages/About.vue") },
    {
        path: "/:pathMatch(.*)*", name: "error", component: () => import("@/pages/Error.vue")
    }
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

router.beforeEach((to, from) => {
    to.meta.transition = "fade";
});
