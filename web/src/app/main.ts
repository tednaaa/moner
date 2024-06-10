import "./scss/main.scss";
import "primeicons/primeicons.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import { MotionPlugin } from "@vueuse/motion";

import PrimeVue from "primevue/config";
import ToastService from "primevue/toastservice";
import ConfirmationService from "primevue/confirmationservice";

import App from "./app.vue";

import { pt } from "@/shared/ui";
import { router } from "@/pages/router";

const app = createApp(App);

app.use(PrimeVue, { unstyled: true, pt });
app.use(ToastService);
app.use(ConfirmationService);

app.use(MotionPlugin);
app.use(createPinia());
app.use(router);

app.mount("#app");
