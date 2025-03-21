import type { ProductRecord, User } from "@/model";

// NOTE: 
// use this key type to call real invoke  
// make sure use same name as invoke in rust 

export type InvokeKeyType =
  'product_command_get_products'

export interface InvokeCommand {
  getProductRecord: () => Promise<ProductRecord>,
  getUserById: (id: number) => Promise<User>,
  getUsers: () => Promise<User[]>
}
