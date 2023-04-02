import MainApp from "./App.vue";
import { App, createApp } from "vue";
import { registerPlugins } from "@/plugins";

import "primevue/resources/primevue.min.css"
import "primeicons/primeicons.css";
import "primeflex/primeflex.css";
import "@/styles/global.scss";
import "@/styles/theme.css";

const authenticated = <HTMLScriptElement>document.querySelector("#app");
let app: App;
if (authenticated === null) {
    app = createApp(MainApp);
} else {
    app = createApp(MainApp, { ...authenticated!.dataset });
}

registerPlugins(app);

app.mount("#app");

export default app;