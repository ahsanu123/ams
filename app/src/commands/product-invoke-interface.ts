import type { Product, User } from "@/model";

// NOTE: 
// use this key type to call real invoke  
// make sure use same name as invoke in rust 

export type ProductInvokeKey =
  'product_command_get_products'

export interface ProductInvokeCommand {
  getProductRecord: () => Promise<Array<Product>>,
  getProductPrice: () => Promise<number>,
  getAllProductOfThisMonth: (date: Date) => Promise<Array<Product>>
}

