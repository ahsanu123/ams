import { invoke, InvokeArgs, InvokeOptions } from '@tauri-apps/api/core';

function mockInvoke(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<any> {
  switch (cmd) {
    case "hello":
      return Promise.resolve(`hello`)
      break;

    default:
      return Promise.reject("unknown command!!")
      break;
  }
}


export function getInvoke() {
  // @ts-ignore
  if (!window.__TAURI__) {
    console.log("running on browser")
  }
  else {
    console.log("running on tauri app")
  }
  // @ts-ignore
  return window.__TAURI__ ? invoke : mockInvoke
}
