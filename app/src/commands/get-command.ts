// function mockInvoke(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<any> {
//   return Promise.resolve({})
// }

import type { ProductRecord, User } from "@/model"
import type { InvokeCommand } from "./invoke-interface"
import { mockCommand } from "./mock-invoke"

// @ts-ignore
export const isInsideTauri = import.meta.env.VITE_ENV == "tauri"
export const viteEnv = import.meta.env.VITE_ENV

// TODO: real invoke 
export const realCommand: InvokeCommand = {
  getProductRecord: function (): Promise<ProductRecord> {
    throw new Error("Function not implemented.")
  },
  getUserById: function (id: number): Promise<User> {
    throw new Error("Function not implemented.")
  },
  getUsers: function (): Promise<User[]> {
    throw new Error("Function not implemented.")
  }
}

export const getCommand = (): InvokeCommand => {
  if (isInsideTauri) return realCommand
  else return mockCommand
}
