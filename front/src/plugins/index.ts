import router from "../router";
import PrimeVue from 'primevue/config';
import ToastService from 'primevue/toastservice';

import { App } from "vue";

export function registerPlugins(app: App) {
  app.use(PrimeVue).use(router).use(ToastService);
}
