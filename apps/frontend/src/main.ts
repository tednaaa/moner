import './assets/styles/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(PrimeVue, {
  theme: {
    preset: 'none',
    options: {
      cssLayer: { name: 'primevue', order: 'base, primevue' },
      darkModeSelector: '.dark',
    },
  },
})

app.mount('#app')
