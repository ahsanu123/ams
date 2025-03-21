import type { Product, ProductRecord } from "./product-record";

export interface DialogModel {
  date: Date,
  data?: Product,
  otherData: ProductRecord,
}
