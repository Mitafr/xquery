import { createRouter, createWebHistory } from "vue-router";
import { store } from "@/auth";
import { Breadcrumb, BreadcrumbItem } from "./breadcrumb";

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
        meta: { breadcrumb: new Breadcrumb([new BreadcrumbItem("Map", "pi pi-map")]) }
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
        meta: { breadcrumb: new Breadcrumb([new BreadcrumbItem("Query", "pi pi-search")]) }
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
        meta: { breadcrumb: new Breadcrumb([new BreadcrumbItem("Stats", "pi pi-chart-bar")]) }
      },
    ],
  },
  {
    path: "/logs",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "Logs",
        component: () =>
          import(/* webpackChunkName: "home" */ "@/views/admin/Logs.vue"),
        meta: { breadcrumb: new Breadcrumb([new BreadcrumbItem("Logs", "pi pi-wrench")]) }
      },
    ],
  },
  {
    path: "/admin",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "Admin",
        component: () =>
          import(/* webpackChunkName: "home" */ "@/views/admin/Admin.vue"),
        meta: { breadcrumb: new Breadcrumb([new BreadcrumbItem("Admin", "pi pi-users")]) }
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
