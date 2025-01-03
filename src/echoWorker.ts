import * as wasm from '../pkg/wasm.js';

//console.log(wasm);

onmessage = msgHandler;
onerror = errHandler;
onmessageerror = msgError;


async function msgHandler(e: MessageEvent<wasm.WorkerRequest>) {
    console.log(e);
    const data = e.data;
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