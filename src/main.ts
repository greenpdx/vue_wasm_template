import './assets/main.css'

import * as wasm from '../pkg'

const env = import.meta.env
const conf = wasm.init(env)

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

console.log(wasm)
export { wasm, conf }


const TheApp = {
  template: `<Suspense><App /></Suspense>`,
  components: { App },
}

const app = createApp(TheApp)

app.use(createPinia())
app.use(router)
app.provide('wasm', {wasm, conf})
app.mount('#app')
