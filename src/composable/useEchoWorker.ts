import type { MaybeRef } from 'vue'

import { ref, inject, watch } from 'vue'
import { echoWorkerKey } from '../injectionKeys'

export const useEchoWorker = (_msg: MaybeRef<string>) => {
  const msg = ref(_msg)

  const rply = ref<string | null>(msg.value)
  const fetching = ref(false)

  const echoWorker = inject(echoWorkerKey)

  watch(
    [msg],
    async () => {
      fetching.value = true

      if (echoWorker) {
        const rawMsg = JSON.parse(JSON.stringify(msg.value))
        console.log('before')
        rply.value = await echoWorker.echoAsync(rawMsg)
        console.log('after')
      } else {
        throw new DOMException(
          'Worker is not defined. Check that you have properly installed the worker plugin.'
        )
      }

      fetching.value = false
    },
    { immediate: true }
  )

  return {
    rply,
    fetching,
  }
}