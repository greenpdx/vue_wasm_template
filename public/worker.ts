try {
  importScripts('../pkg/wasm_bg.js') // * as wasm from '../pkg'
} catch (e) {
  console.log('W', e)
}

console.log('Initializing worker', self)

// In the worker, we have a different struct that we want to use as in
// `index.js`.
const NumberEval = wasm_bindgen //wasm.NumberEval

async function init_wasm_in_worker() {
  // Load the Wasm file by awaiting the Promise returned by `wasm_bindgen`.
  //await wasm_bindgen('./pkg/wasm_in_web_worker_bg.wasm');
  console.log('Init')

  // Create a new object of the `NumberEval` struct.
  const num_eval = NumberEval.new()

  // Set callback to handle messages passed to the worker.
  self.onmessage = async (event) => {
    // By using methods of a struct as reaction to messages passed to the
    // worker, we can preserve our state between messages.
    const worker_result = num_eval.is_even(event.data)

    // Send response back to be handled by callback in main thread.
    self.postMessage(worker_result)
  }
}
console.log('Test')
init_wasm_in_worker()
