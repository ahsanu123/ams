import type { Product, User } from "@/model"
import type { ProductInvokeCommand } from "./product-invoke-interface"
import { mockProductCommand } from "@/mock"
import { IS_INSIDE_TAURI } from "@/constants"

const realCommand: ProductInvokeCommand = {
  getProductRecord: function (): Promise<Array<Product>> {
    throw new Error("Function not implemented.")
  },
  getProductPrice: function (): Promise<number> {
    throw new Error("Function not implemented.")
  },
  getAllProductOfThisMonth: function (date: Date): Promise<Array<Product>> {
    throw new Error("Function not implemented.")
  }
}

export const getProductCommand = (): ProductInvokeCommand => {
  if (IS_INSIDE_TAURI) return realCommand
  else return mockProductCommand
}
