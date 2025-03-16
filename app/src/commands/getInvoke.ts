import type { ProductRecord } from "@/model"
import { invoke as tauriInvoke, type InvokeArgs, type InvokeOptions } from "@tauri-apps/api/core"

export type ProductRecordInvokeCommand =
  'get_products'

export interface ProductRecordCommand {

  getProducts: () => Promise<ProductRecord>

}

function mockInvoke(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<any> {
  return Promise.resolve({})
}

// @ts-ignore
export const isInsideTauri = import.meta.env.VITE_ENV == "tauri"
export const viteEnv = import.meta.env.VITE_ENV
// const invoke = insideTauriEnvironment ? tauriInvoke : mockInvoke


export function whatEnvironmentIs() {
  // return insideTauriEnvironment ? "TAURI" : "BROWSER"
}


