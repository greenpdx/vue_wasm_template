import * as wasm from '../pkg/wasm.js';

//console.log(wasm);

onmessage = msgHandler;
onerror = errHandler;
onmessageerror = msgError;

console.log(onmessage)

async function msgHandler(e: MessageEvent<wasm.WorkerRequest>) {
    const data = e.data;
    console.log(data);
    const response: wasm.WorkerResponse = await wasm.wtest(data);
    //console.log(response)
    postMessage(response);
}

function errHandler(evt: Event | string) {
    console.log("Worker Error", evt);
}

function msgError(evt: Event | string) {
    console.log("Msg Error", evt);
}






//onmessage = msgHandler;
//onerror = errHandler;
//onmessageerror = msgError;