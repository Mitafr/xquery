<template>
    <div class="col-12 p-fluid m-auto">
        <ol-map :loadTilesWhileAnimating="true" :loadTilesWhileInteracting="true" :style="{ height: height + 'px' }">
            <ol-view ref="view" :center="center" :rotation="rotation" :zoom="zoom" :projection="projection" />
            <ol-tile-layer>
                <ol-source-osm />
            </ol-tile-layer>
        </ol-map>
    </div>
</template>
  
<script lang="ts" setup>
import { App, ref } from 'vue';
import 'vue3-openlayers/dist/vue3-openlayers.css'
import OpenLayersMap from 'vue3-openlayers'

import app from "@/main";

app.use(OpenLayersMap);
const projection = ref('EPSG:3857')
const zoom = ref(5)
const rotation = ref(0)
const center = ref([2.812, 48.862])

const height = ref(window.innerHeight - 100);

window.onresize = () => {
    height.value = window.innerHeight - 100
}

</script>
  