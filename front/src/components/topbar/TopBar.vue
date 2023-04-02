<template>
  <div id="topbar"
    class="flex justify-content-between align-items-center px-2 py-1 static border-bottom-1 surface-border shadow-3">
    <Button icon="pi pi-bars" class="p-button-text outline-none" style="box-shadow: none;" @click="emit('hideNav')" />
    <div class="flex"></div>
    <AutoComplete placeholder="Search..." class="w-4" inputClass="w-12" :inputStyle="searchStyle" />
    <div class="flex"></div>
    <span class="p-buttonset flex align-items-center justify-content-center">
      <MenuSettings />
      <MenuAvatar :label="store.user.username" class="mr-2" style="background-color:#9c27b0; color: #ffffff"
        :user="auth.user" />
    </span>
  </div>
</template>

<script lang="ts" setup>
import { store } from "@/auth";
import { useRouter } from "vue-router";
import Button from 'primevue/button';
import AutoComplete from 'primevue/autocomplete';
import MenuAvatar from './MenuAvatar.vue';
import MenuSettings from './MenuSettings.vue';

const auth = store;
const router = useRouter();

const logout = function () {
  auth.postLogout().then(() => {
    router.push("/login");
  });
}

const emit = defineEmits(["hideNav"]);

const searchStyle = {
  height: "35px"
}

</script>
<style scoped>
#topbar {
  background-color: #25282f;
  border-bottom-color: #25282f;
  height: 48px;
}
</style>