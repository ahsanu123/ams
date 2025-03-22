import type { Product, ProductRecord } from "./product-record";
import type { User } from "./user-model";

export interface DialogModel {
  date: Date,
  product: Product,
  user: User,
}
