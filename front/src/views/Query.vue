<template>
    <div class="col-12 xl:col-10 p-fluid m-auto">
        <div class="surface-card p-4 shadow-2 border-round w-full grid">
            <div class="col-6">
                <QueryInput type="text" v-model="params.oid" label="Record ID" @enter="launchQuery" />
            </div>
            <div class="col-6">
                <QueryInput type="text" v-model="params.name" label="Nom" @enter="launchQuery" />
            </div>
            <div class="col-12">
                <div class="formgrid grid">
                    <div class="field md:col-6 lg:col-6 sm:col-12 xl:col-3 xl:col-offset-3">
                        <Button label="Cancel" icon="pi pi-times" iconPos="right"
                            class="p-button p-button-rounded p-button-warning" />
                    </div>
                    <div class="field md:col-6 lg:col-6 sm:col-12 xl:col-3">
                        <Button label="Search" icon="pi pi-search" iconPos="right"
                            class="p-button p-button-rounded p-button-success" :loading="searchLoading"
                            @click="launchQuery" />
                    </div>
                </div>
            </div>
        </div>
        <div class="card mt-3 surface-card p-4 shadow-2 border-round w-full" v-show="products.length > 0">
            <DataView :value="products" layout="list" dataKey="results" :paginator="true" :rows="10">
                <template #header>
                    <div class="grid grid-nogutter">
                        <div class="col-6" style="text-align: right">
                        </div>
                    </div>
                </template>

                <template #list="slotProps">
                    <QueryResult :result="slotProps.data"></QueryResult>
                </template>

            </DataView>
        </div>
    </div>
</template>
  
<script lang="ts" setup>
import QueryInput from '@/components/query/QueryInput.vue';
import Button from 'primevue/button';
import DataView from 'primevue/dataview';
import QueryResult from '@/components/query/QueryResult.vue';
import { Ref, ref } from 'vue';

export interface IQueryProps {
    oid: string,
    name?: string,
}

class Result {
    name: string;

    constructor(name: string) {
        this.name = name;
    }
}

class QueryParams implements IQueryProps {
    oid: string;
    name?: string;

    constructor(oid: string) {
        this.oid = oid;
    }
}

const params = ref(new QueryParams(""))
const products: Ref<Result[]> = ref([]);

const searchLoading = ref(false);

const launchQuery = async () => {
    console.log(params);
    searchLoading.value = true;
    products.value = [
        new Result("Bamboo Watch"),
        new Result("Black Watch")
    ];
}


</script>
  
<style lang="scss" scoped>
.card {
    background: #ffffff;
    padding: 2rem;
    box-shadow: 0 2px 1px -1px rgba(0, 0, 0, .2), 0 1px 1px 0 rgba(0, 0, 0, .14), 0 1px 3px 0 rgba(0, 0, 0, .12);
    border-radius: 4px;
    margin-bottom: 2rem;
}

.p-dropdown {
    width: 14rem;
    font-weight: normal;
}
</style>