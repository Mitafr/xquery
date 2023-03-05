<template>
  <div id="topbar"
    class="flex justify-content-between align-items-center px-2 surface-section static border-bottom-1 surface-border shadow-3">
    <Button icon="pi pi-bars" class="p-button-raised p-button-text" @click="emit('hideNav')" />
    <div class="flex"></div>
    <ul
      class=" list-none p-0 m-0 flex lg:align-items-center select-none flex-row border-none surface-border right-0 top-100 z-1 shadow-none static">
      <li class="border-1 surface-border">
        <SplitButton :label="store.user.username" icon="pi pi-user" :model="profileItems"
          class="p-button-text p-button-sm">
        </SplitButton>
      </li>
    </ul>
  </div>
</template>

<script lang="ts" setup>
import { store } from "@/auth";
import { useRouter } from "vue-router";
import Button from 'primevue/button';
import SplitButton from 'primevue/splitbutton';

const auth = store;
const router = useRouter();

const logout = function () {
  auth.postLogout().then(() => {
    router.push("/login");
  });
}

const emit = defineEmits(["hideNav"]);

const profileItems = [
  {
    label: 'Voir plus',
    icon: 'pi pi-refresh'
  },
  {
    separator: true
  },
  {
    label: 'Logout',
    icon: 'pi pi-sign-out',
    command: () => {
      logout()
    }
  },
];
</script>