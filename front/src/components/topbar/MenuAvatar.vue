<template>
    <Avatar :label="label.substring(0, 2)" shape="circle" @click="toggle"
        style="background-color: rgba(0,0,0,0);cursor: pointer; color: #ffffff">
    </Avatar>

    <Sidebar position="right" v-model:visible="visible" modal header="Profile">
        <p>
            {{ user.username }}
        </p>
        <p>
            {{ user.profile }}
        </p>
    </Sidebar>
</template>

<script lang="ts" setup>
import User from '@/auth/types/user';
import Avatar from 'primevue/avatar';
import Sidebar from 'primevue/sidebar';
import { ref } from "vue";

interface IMenuAvatarProps {
    readonly label: string,
    readonly user: User,
}

const visible = ref(false);

defineProps<IMenuAvatarProps>();

const toggle = () => {
    visible.value = true;
}

</script>
<style scoped></style>