<template>
  <div class="col-12 xl:col-10 p-fluid m-auto">
    <h2>Home</h2>
    <div class="p-4 border-round w-full grid">
      <div class="col-6">
        <Card>
          <template #title> Map </template>
          <template #content>
            <p>Voir la map</p>
          </template>
          <template #footer>
            <Button icon="pi pi-check pi-button-sm" label="Map" @click="$router.push('/map')" />
          </template>
        </Card>
      </div>
      <div class="col-6">
        <Card>
          <template #title> Query </template>
          <template #content>
            <p>Query</p>
          </template>
          <template #footer>
            <Button icon="pi pi-check pi-button-sm" label="Query" @click="$router.push('/query')" />
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
