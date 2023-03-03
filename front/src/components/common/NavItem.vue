<template>
    <li class="hover:surface-100 transition-duration-150 transition-colors p-ripple" :class="selected ? 'surface-100' : ''">
        <router-link :to="to ?? ''" @click="emit('click')"
            class="flex flex-row lg:flex-row align-items-center cursor-pointer px-5 text-600 border-right-3 border-transparent hover:border-200 transition-duration-150 transition-colors p-ripple"
            :class="selected ? 'border-right-3' : ''" style="text-decoration: none" type="button">
            <i class="text-base lg:text-2xl mr-4 my-3" :class="icon" style="font-size: 1.4rem;"></i>
            <span class="font-medium inline text-base lg:text-xs lg:block" :class="selected ? 'font-bold' : ''">{{ alias
            }}</span>
        </router-link>
    </li>
</template>
  
<script lang="ts" setup>

defineProps<INavItemProps>();

const emit = defineEmits(["click"]);

</script>

<script lang="ts">

export interface INavItemProps {
    readonly alias: string,
    readonly to: string | null,
    readonly icon: string | null,
    selected: boolean
}

export class NavItemProps implements INavItemProps {
    alias: string;
    to: string | null;
    icon: string | null;
    selected: boolean;

    constructor(alias: string, to: string | null, icon: string | null, selected: boolean) {
        this.alias = alias;
        this.to = to;
        this.icon = icon;
        this.selected = selected;
    }
}


</script>
  