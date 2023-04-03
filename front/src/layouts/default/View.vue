<template>
  <div class="min-h-screen flex relative lg:static surface-ground">
    <NavBar :show="showNav" @toggleNav="toggleNav"></NavBar>
    <div id="content" class="min-h-screen  flex flex-column relative flex-auto" style="height: 60px">
      <TopBar @hideNav="toggleNav"></TopBar>
      <TopBarBreadcrumb @hideNav="toggleNav"></TopBarBreadcrumb>
      <div class="m-2">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import TopBar from "@/components/topbar/TopBar.vue";
import TopBarBreadcrumb from "@/components/topbar/TopBarBreadcrumb.vue";
import NavBar from "@/components/common/NavBar.vue";
import { ref } from "vue";

const showNav = ref(false)

const toggleNav = (e?: boolean) => {
  if (e !== undefined) {
    showNav.value = e;
  } else {
    showNav.value = !showNav.value;
  }
}

const hideNav = () => {
  showNav.value = false;
}

const hideNavIfNeeded = () => {
  if (needHide(window.innerWidth)) {
    hideNav();
  }
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

hideNavIfNeeded();

</script>

<style scoped></style>