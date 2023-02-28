<template>
    <div>
        <label for="username" class="text-900 font-medium mb-2">{{ label }}</label>
        <InputText id="username" type="text" v-model="innerValue" class="p-inputtext-sm w-full mb-3"
            @keyup.enter="$emit('enter')"
            @input="$emit('update:modelValue', ($event!.target! as HTMLInputElement).value)" />
    </div>
</template>
  
<script lang="ts" setup>
import { ref, watch } from 'vue';
import InputText from 'primevue/inputtext';

interface Props {
    label: string,
    modelValue?: string
}

const emit = defineEmits(["enter", 'update:modelValue']);
const props = defineProps<Props>();

const innerValue = ref(props.modelValue);

watch(() => props.modelValue, () => {
    innerValue.value = props.modelValue
});

</script>
  