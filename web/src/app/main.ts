import './scss/main.scss'
import 'primeicons/primeicons.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { MotionPlugin } from '@vueuse/motion';
import PrimeVue from 'primevue/config';
import ToastService from 'primevue/toastservice';
import Tooltip from 'primevue/tooltip';

import App from './app.vue'

import { router } from '@/pages/router'
import { designSystem } from '@/shared/ui';

const app = createApp(App)

app.use(PrimeVue, { unstyled: true, pt: designSystem })
app.use(MotionPlugin)
app.use(ToastService)
app.directive('tooltip', Tooltip);

app.use(createPinia())
app.use(router)

app.mount('#app')
