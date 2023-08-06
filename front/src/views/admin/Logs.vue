<template>
    <div class="col-12 xl:col-10 p-fluid m-auto">
        <div class="surface-card p-4 shadow-2 border-round w-full grid">
            <div class="col-12">
                <span>Logs</span>
            </div>
            <div class="col-12 justify-content-center">
                <Paginator :rows="20" :totalRecords="logs?.total" @page="changePage"></Paginator>
            </div>

            <div class="col-12 justify-content-center" v-show="logs?.results">
                <DataTable :value="logs?.results" tableStyle="min-width: 50rem">
                    <Column v-for="col of columns" :key="col.field" :field="col.field" :header="col.header"></Column>
                </DataTable>
            </div>
        </div>
    </div>
</template>
<script lang="ts" setup>
import { getLogs } from "@/services/admin/index";
import { AxiosResponse } from "axios";
import { Ref, onMounted, ref } from "vue";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import ColumnGroup from 'primevue/columngroup';   // optional
import Row from 'primevue/row';                   // optional
import Paginator, { PageState } from 'primevue/paginator';

interface LogResult {
    results: any[],
    total: number,
}

onMounted(() => {
    getLogs().then((res: AxiosResponse<any, any>) => (logs.value = res.data));
});

const columns = [
    { field: 'level', header: 'Level' },
    { field: 'description', header: 'description' },
    { field: 'created_at', header: 'Timestamp' }
];

let logs: Ref<LogResult | undefined> = ref();

const changePage = async function (event: PageState) {
    logs.value = (await getLogs(event.page)).data;
}

</script>
  