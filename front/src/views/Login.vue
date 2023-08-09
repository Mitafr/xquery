<template>
  <div class="grid h-screen w-screen m-0">
    <AuthLogin @login="sendLogin" :loading="loading"></AuthLogin>
    <Toast position="top-center" />
  </div>
</template>

<script lang="ts" setup>
import AuthLogin from "@/components/security/AuthLogin.vue";
import { ref } from "vue";
import { store } from "@/auth";
import { useRouter } from "vue-router";
import { useToast } from "primevue/usetoast";
import Toast from "primevue/toast";
import axios from "axios";

let loading = ref(false);

const auth = store;
const router = useRouter();
const toast = useToast();

const sendLogin = async (payload: any) => {
  try {
    loading.value = true;
    await auth.postLogin(payload.username, payload.password);
  } catch (err) {
    if (axios.isAxiosError(err) && err.response) {
      toast.removeAllGroups();
      toast.add({ severity: 'error', summary: err.response.data, life: 6000 });
    }
  } finally {
    loading.value = false;
    // auth.user.authenticated = true;
    router.push("/");
  }
}

if (auth.user.authenticated) {
  router.push("/");
}

</script>
