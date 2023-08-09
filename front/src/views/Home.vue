<template>
  <div class="col-12 xl:col-10 p-fluid m-auto">
    <h2>Home</h2>
    <div class="p-4 w-full grid">
      <div class="col-12 md:col-6">
        <Card class="border-round hover:surface-100" @click="$router.push('/query')" style="cursor: pointer;">
          <template #title> Query </template>
          <template #content>
            <p>Query</p>
          </template>
        </Card>
      </div>
      <div class="col-12 md:col-6">
        <Card class="border-round hover:surface-100" @click="$router.push('/map')" style="cursor: pointer;">
          <template #title> Map </template>
          <template #content>
            <p>View Map</p>
          </template>
        </Card>
      </div>
      <div class="col-12 md:col-6">
        <Card class="border-round hover:surface-100" @click="$router.push('/logs')" style="cursor: pointer;">
          <template #title> Logs </template>
          <template #content>
            <p>Logs</p>
          </template>
        </Card>
      </div>
      <div class="col-12 md:col-6">
        <Card class="border-round hover:surface-100" @click="$router.push('/admin')" style="cursor: pointer;">
          <template #title> Admin </template>
          <template #content>
            <p>Administration Panel</p>
          </template>
        </Card>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Button from "primevue/button";
import Card from "primevue/card";

export default defineComponent({
  name: "Home",
  data() {
    return {};
  },
  components: {
    Card,
    Button
  },
  methods: {},
});
</script>
