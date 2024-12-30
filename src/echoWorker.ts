import * as wasm from '../pkg/wasm.js'

console.log(wasm)
export type WorkerRequest = wasm.WorkerRequest;
export type WorkerResponse = wasm.WorkerResponse;

addEventListener("message", (e: MessageEvent<WorkerRequest>) => {
    console.log(e)
    const data = e.data;
    const response = wasm.wtest(data);
    console.log(response)
    postMessage(response);
})