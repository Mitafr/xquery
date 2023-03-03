<template>
  <div class="min-h-screen flex relative lg:static surface-ground">
    <NavBar :show="showNav" @hideNav="hideNav"></NavBar>
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
  console.log(showNav.value)
}

const hideNav = () => {
  showNav.value = false;
}

const onResize = (e: Event) => {
  if (needHide((e.target as Window).innerWidth)) {
    hideNav();
  }
}

window.addEventListener("resize", onResize);

const needHide = (width: number) => {
  return (width <= 700);
}

if (needHide(window.innerWidth)) {
  hideNav();
}

</script>
