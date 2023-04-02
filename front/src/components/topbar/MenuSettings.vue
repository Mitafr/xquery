<template>
    <Button icon="pi pi-cog" text size="small" aria-label="Settings" @click="toggle" />
    <Sidebar v-model:visible="visible" modal header="Settings" position="right">
    </Sidebar>
</template>

<script lang="ts" setup>
import Button from 'primevue/button';
import Sidebar from 'primevue/sidebar';
import { ref } from "vue";

const visible = ref(false);

const toggle = () => {
    visible.value = true;
}

</script>
<style scoped></style>