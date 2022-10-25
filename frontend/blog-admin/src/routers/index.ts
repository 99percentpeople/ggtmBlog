import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import { useLoginStatus } from "@/stores";
const routes = [
    {
        path: "/",
        redirect: { name: "admin" },
    },
    {
        path: "/admin",
        name: "admin",
        component: () => import("@/pages/admin/Admin.vue"),
        children: [
            { path: "", name: "home", component: () => import("@/pages/admin/Home.vue") },
            { path: "sort", name: "sort", component: () => import("@/pages/admin/Sort.vue") },
            { path: "tag", name: "tag", component: () => import("@/pages/admin/Tag.vue") },
            { path: "file", name: "file", component: () => import("@/pages/admin/File.vue") },
            {
                path: "publish/:blog_id(\\d+)?",
                name: "publish",
                component: () => import("@/pages/admin/Publish.vue"),
            },
        ] as RouteRecordRaw[],
        meta: {
            requireAuth: true,
        },
        redirect: { name: "home" },
    },

    { path: "/login", name: "login", component: () => import("@/pages/login/Login.vue") },
] as RouteRecordRaw[];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

router.beforeEach((to, from, next) => {
    const loginStatus = useLoginStatus();

    to.meta.transition = "fade";

    if (to.name == "login") {
        if (loginStatus.loginStatus) {
            return next((cb) => cb.$router.back());
        } else {
            return next();
        }
    } else if (to.meta.requireAuth) {
        if (loginStatus.loginStatus) {
            // if (to.meta.AccessLevel && loginStatus.userInfo.accessLevel) {
            //     if (to.meta.AccessLevel <= loginStatus.userInfo.accessLevel) return next();
            //     else return next({ name: "login" });
            // } else {
            //     return next();
            // }
            return next();
        } else {
            return next({ name: "login" });
        }
    } else {
        return next();
    }
});
