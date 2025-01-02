import type { Plugin } from "vue";
import type { WorkerResponse, WorkerRequest } from "./echoWorker";

import EchoWorker from "./echoWorker?worker";
import { echoWorkerKey } from "./injectionKeys.ts";

type WorkerPluginOptions = {
  minWorkers?: number;
  maxWorkers?: number;
};

const plugin: Plugin = {
  install: (app, options: WorkerPluginOptions) => {
    const MIN_WORKERS = options?.minWorkers ?? 1;
    const MAX_WORKERS = options?.maxWorkers ?? navigator.hardwareConcurrency - 1;

    const workers: Worker[] = [];
    const workerPool: Worker[] = [];
    const messageQueue: WorkerRequest[] = [];
    const resolvers: Record<string, (value: any) => void> = {};

    for (let i = 0; i < MIN_WORKERS; i++) {
      addWorker();
    }

    window.onunload = () => {
      for (const worker of workers) {
        worker.terminate();
      }
    };

    function echoAsync(msg: string) {
      const id = Math.random().toString();

      return new Promise<string>((resolve) => {
        resolvers[id] = resolve;

        const request: WorkerRequest = {
          id,
          msg,
        };

        queueMessage(request);
      });
    }

    function queueMessage(query: WorkerRequest) {
      messageQueue.push(query);
      processNextQuery();
    }

    function processNextQuery() {
      adjustWorkerPool();

      if (workerPool.length > 0 && messageQueue.length > 0) {
        const worker = workerPool.shift();
        const msg = messageQueue.shift();

        worker?.postMessage(msg);
      }
    }

    function adjustWorkerPool() {
      if (messageQueue.length > workerPool.length) {
        addWorker();
      } else if (messageQueue.length < workerPool.length) {
        removeWorker();
      }
    }

    async function addWorker() {
      if (workers.length < MAX_WORKERS) {
        const worker = await new EchoWorker();

        worker.addEventListener("message", (event: MessageEvent<WorkerResponse>) => {
          const resolve = resolvers[event.data.id];
          console.log(event.data)
          resolve(event.data.rply);
          delete resolvers[event.data.id];

          workerPool.push(worker);
          processNextQuery();
        });

        workers.push(worker);
        workerPool.push(worker);
      }
    }

    function removeWorker() {
      if (workers.length > MIN_WORKERS) {
        const worker = workerPool.pop();

        if (worker && workers.includes(worker)) {
          workers.splice(workers.indexOf(worker), 1);
        }

        worker?.terminate();
      }
    }

    app.provide(echoWorkerKey, { echoAsync });
  }
};

export default plugin;