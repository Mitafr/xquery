<template>
    <div id="topbar-breadcrumb" class="flex justify-content-between align-items-center px-2 py-1 static border-bottom-1 surface-border shadow-3">
      <Breadcrumb :home="home" :model="items" />
      <div class="flex"></div>
    </div>
</template>
  
<script lang="ts" setup>
import { Ref, ref, watch } from "vue";
import Breadcrumb from 'primevue/breadcrumb';
import { Breadcrumb as RouterBreadcrumb }  from '@/router/breadcrumb';
import { useRoute } from "vue-router";
import { MenuItem } from "primevue/menuitem";

const route = useRoute();

const home = ref({
    icon: 'pi pi-home',
    to: '/',
});
const items: Ref<MenuItem[]> = ref([]);

watch(
  () => route.meta,
  async () => {
    if (route.meta.hasOwnProperty("breadcrumb")) {
        items.value = (route.meta.breadcrumb as RouterBreadcrumb).breadcrumbs;
    } else {
        items.value = [];
    }
  }
);
</script>
<style scoped>
#topbar-breadcrumb {
height: 48px;
}
#topbar-breadcrumb .p-breadcrumb {
    background: none !important;
    border: none !important;
}
</style>