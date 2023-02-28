import { createRouter, createWebHistory } from "vue-router";
import { store } from "@/auth";

const routes = [
  {
    path: "/",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "Home",
        component: () =>
          import(/* webpackChunkName: "home" */ "@/views/Home.vue"),
      },
    ],
  },
  {
    path: "/map",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "Map",
        component: () =>
          import(/* webpackChunkName: "home" */ "@/views/Map.vue"),
      },
    ],
  },
  {
    path: "/query",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "Query",
        component: () =>
          import(/* webpackChunkName: "home" */ "@/views/Query.vue"),
      },
    ],
  },
  {
    path: "/stats",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "Stats",
        component: () =>
          import(/* webpackChunkName: "home" */ "@/views/Stats.vue"),
      },
    ],
  },
  {
    name: "Login",
    path: "/login",
    component: () => import(/* webpackChunkName: "home" */ "@/views/Login.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

router.beforeEach(async (to, from, next) => {
  const isAuthenticated = store.user.authenticated;
  if (!isAuthenticated && to.path !== "/login") {
    return next({ name: "Login" });
  }
  next()
});

export default router;
