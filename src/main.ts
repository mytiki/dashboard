import { createApp } from 'vue'
import PrimeVue from '@primevue/core/config'
import App from './App.vue'
import Aura from '@primevue/themes/aura'
import router from './router'

const app = createApp(App)

app.use(router)

app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      prefix: 'p',
      darkModeSelector: 'selector',
      cssLayer: { name: 'primeVue' }
    }
  }
})

app.mount('#app')
