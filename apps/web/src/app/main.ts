import './scss/main.scss'
import "vue-toastification/dist/index.css";

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { MotionPlugin } from '@vueuse/motion';
import Toast from "vue-toastification";

import App from './app.vue'

import { router } from '@/pages/router'

const app = createApp(App)

app.use(createPinia())
app.use(MotionPlugin)
app.use(Toast);
app.use(router)

app.mount('#app')
