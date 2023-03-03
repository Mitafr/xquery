<template>
  <div class="min-h-screen flex relative lg:static surface-ground">
    <NavBar :show="showNav"></NavBar>
    <div class="min-h-screen flex flex-column relative flex-auto">
      <TopBar @hideNav="toggleNav"></TopBar>
      <div class="m-2">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>

import TopBar from "@/components/common/TopBar.vue";
import NavBar from "@/components/common/NavBar.vue";
import { ref } from "vue";

const showNav = ref(true)

const toggleNav = () => {
  showNav.value = !showNav.value;
}

const onResize = (e: Event) => {
  needHide((e.target as Window).innerWidth)
}

window.addEventListener("resize", onResize);

const needHide = (width: number) => {
  if (width <= 700) {
    showNav.value = false;
  }
}

needHide(window.innerWidth);

</script>
