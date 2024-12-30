import type { InjectionKey } from "vue";

export const echoWorkerKey: InjectionKey<{
  echoAsync: (msg: string) => Promise<string>;
}> = Symbol("echoWorker");
