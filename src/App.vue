<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
import { wasm, conf} from '@/main'
import { onMounted, ref, inject, watch } from 'vue'
import { WorkerRequest } from '../pkg/wasm'

const test = inject('wasm')
console.log(test)

const msg = ref('TEST')
const rply = ref('TEST')

const echoWorker = import.meta.env.DEV
    // In development mode, `import`s in workers are not transformed, so you
    // must use `{ type: "module" }`.
  ? new Worker(new URL("./echoWorker.ts", import.meta.url), { type: "module" })
    // In build mode, let Vite and vite-plugin-top-level-await build a single-file
    // bundle of your worker that works on both modern browsers and Firefox.
  : new Worker(new URL("./echoWorker.ts", import.meta.url), { type: "classic" });

echoWorker.addEventListener("message", (evt: MessageEvent<wasm.WorkerResponse>) => {
  rply.value = evt.data.rply
  console.log(evt.data)
})

watch(
  [msg],
  async () => {
    const rawMsg = JSON.parse(JSON.stringify(msg.value))
    const id = Math.random().toString();
    const req: WorkerRequest =  {id: id, msg: rawMsg}

    await echoWorker.postMessage(req)
  }
)

console.log('p1')

onMounted(() => {
  wasm.tsturl(conf)
  console.log('a')
})
//run_wasm()
</script>

<template>
  <header>
    <input v-model="msg" /><br>
    <div>{{ rply }}</div>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />

    <div class="wrapper">
      <HelloWorld msg="You did it!" />
      <input type="text" id="inputNumber" />

      <div id="resultField"></div>
      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/about">About</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
