<template>
  <Sidebar id="sbar" v-model:visible="showNav" role="region" @hide="emit('toggleNav', false)"
    @show="emit('toggleNav', true)">
    <div class="flex flex-column h-full">
      <img src="@/assets/logo.svg" alt="Image" height="100" width="200" class="my-1" />
      <Divider class="mb-0" />
      <ul class="relative list-none p-0 m-0">
        <NavItem v-for="(item, i) in navItems" :key="i" :alias="item.alias" :icon="item.icon" :to="item.to"
          @click="emit('toggleNav', false)" :selected="item.selected"></NavItem>
      </ul>
      <div class="mt-auto">
        <Divider class="mb-0" />
        <ul class="list-none p-0 m-0">
          <li class="mb-2 hover:surface-100 transition-duration-150 transition-colors p-ripple">
            <a class="flex flex-row lg:flex-column align-items-center cursor-pointer p-3 lg:justify-content-center"
              style="text-decoration: none" type="button" aria-label="Logout" @click="logout">
              <i class="text-base pi pi-sign-out mr-2 lg:mr-0 mb-0 lg:mb-2 lg:text-2xl"></i>
              <span class="font-medium inline text-base lg:text-xs lg:block">Logout</span>
            </a>
          </li>
        </ul>
      </div>
    </div>
  </Sidebar>
</template>

<script lang="ts" setup>
import Divider from "primevue/divider";
import Sidebar from "primevue/sidebar";
import NavItem from "./NavItem.vue"

import { store } from "@/auth";
import { useRouter, useRoute } from "vue-router";
import { NavItemProps } from "./NavItem.vue"
import { PrimeIcons } from 'primevue/api';
import { ref, watch } from "vue";

const auth = store;
const router = useRouter();
const route = useRoute();

const props = defineProps({
  show: {
    type: Boolean,
  }
});

const showNav = ref(props.show);

const emit = defineEmits(["toggleNav"]);

const logout = async function () {
  await auth.postLogout();
  router.push("/login");
}

watch(() => route.name, () => {
  navItems.value.forEach((i) => {
    i.selected = i.alias === route.name;
  })
});

watch(() => props.show, (oldValue, newValue) => {
  if (newValue) {
    setTimeout(() => { showNav.value = !newValue; }, 100)
  } else {
    showNav.value = true;
  }
});

const navItems = ref([
  new NavItemProps("Home", "/", PrimeIcons.HOME, route.name === "Home"),
  new NavItemProps("Query", "/query", PrimeIcons.SEARCH, route.name === "Query"),
  new NavItemProps("Map", "/map", PrimeIcons.MAP_MARKER, route.name === "Map"),
  new NavItemProps("Stats", "/stats", PrimeIcons.CHART_BAR, route.name === "Stats"),
  new NavItemProps("Logs", "/logs", PrimeIcons.DATABASE, route.name === "Logs"),
  new NavItemProps("Admin", "/admin", PrimeIcons.USERS, route.name === "Admin"),
]);

</script>
<style scoped>
#app-sidebar.collapsing {
  -webkit-transition: left 0.1s ease;
  -o-transition: left 0.1s ease;
  -moz-transition: left 0.1s ease;
  transition: left 0.1s ease;
  left: -100% !important;
}

#app-sidebar.show {
  left: 0 !important;
  -webkit-transition: left 0.1s ease-in;
  -o-transition: left 0.1s ease-in;
  -moz-transition: left 0.1s ease-in;
  transition: left 0.1s ease-in;
}
</style>