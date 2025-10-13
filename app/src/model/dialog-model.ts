import type { TakingRecordModel } from "@/api-models";
import type { User } from "./user-model";

export interface DialogModel {
  date: Date,
  product: TakingRecordModel,
  user: User,
}
