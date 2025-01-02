import * as wasm from '../pkg/wasm.js'

console.log(wasm)

addEventListener("message", (e: MessageEvent<wasm.WorkerRequest>) => {
    console.log(e)
    const data = e.data;
    const response: wasm.WorkerResponse = wasm.wtest(data);
    console.log(response)
    postMessage(response);
})