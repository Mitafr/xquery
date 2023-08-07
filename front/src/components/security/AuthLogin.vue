<template>
  <div class="col-12 md:col-8 lg:col-6 xl:col-4 p-fluid m-auto">
    <div class="surface-card p-4 shadow-8 border-round w-full">
      <div class="text-center mb-5">
        <img src="@/assets/logo.svg" alt="Image" height="80" class="mb-3" />
        <div class="text-900 text-3xl font-medium mb-3">Connexion</div>
        <Divider />
      </div>
      <div class="justify-content-center">
        <label for="username" class="block text-900 font-medium mb-2">Username</label>
        <InputText id="username" type="text" v-model="username" class="w-full mb-3" @keyup.enter="login" />
        <label for="password" class="block text-900 font-medium mb-2">Password</label>
        <Password id="password" v-model="password" toggleMask class="w-full mb-3" @keyup.enter="login"
          :feedback="false" />
        <div class="formgrid grid">
          <div class="field col m-auto md:col-8 sm:col-12">
            <Button label="Submit" icon="pi pi-send" iconPos="right" class="p-button p-button-rounded p-button-success"
              @click="login" :loading="isLoading" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { toRef } from "vue";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Divider from 'primevue/divider';
import Password from 'primevue/password';

interface Props {
  loading: boolean
}

let username = "";
let password = "";
const emit = defineEmits(["login"]);
const props = defineProps<Props>();
const isLoading = toRef(props, 'loading')

const login = async function () {
  if (!validateCredentials()) {
    return;
  }
  emit("login", {
    username: username,
    password: password,
  });
}

const validateCredentials = function () {
  return username.length > 0 && password.length > 0;
}

</script>

<style lang="scss" scoped>
::v-deep(.p-password input) {
  width: 100%;
}
</style>