<template>
  <div class="layout-content">
    <div class="layout-content-inner">
      <router-view></router-view>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { store } from "@/auth";
import { useRouter, useRoute } from "vue-router";

export default defineComponent({
  props: {
    authenticated: {
      type: String,
    },
    username: {
      type: String
    }
  },
  computed: {},
  data() {
    return {
      drawer: true,
    };
  },
  created() {
    if (this.authenticated === "true") {
      this.store.user.authenticate(this.username as string);
      if (this.route.name == "Login") {
        this.router.push({ name: "Home" });
      }
    }
  },
  setup() {
    return {
      router: useRouter(),
      route: useRoute(),
      store: store
    };
  },
});
</script>

<style>
body {
  margin: 0;
  padding: 0;
}
</style>