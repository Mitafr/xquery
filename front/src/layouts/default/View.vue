<template>
  <div class="min-h-screen flex relative lg:static surface-ground">
    <NavBar></NavBar>
    <div class="min-h-screen flex flex-column relative flex-auto">
      <TopBar></TopBar>
      <div class="m-2">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>

import TopBar from "@/components/common/TopBar.vue";
import NavBar from "@/components/common/NavBar.vue";

</script>
