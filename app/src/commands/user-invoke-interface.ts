import type { Product, User } from "@/model";

// NOTE: 
// use this key type to call real invoke  
// make sure use same name as invoke in rust 

export type UserInvokeKey =
  'user_command_get_products'

export interface UserInvokeCommand {
  getUserById: (id: number) => Promise<User>,
  getUsers: () => Promise<User[]>,
}

