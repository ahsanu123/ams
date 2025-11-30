import type { DetailInformation, TakingRecordWithPrice } from "./make-payment-page-model";
import type { UserModel } from "./user-model";

export interface TakingRecordModel {
  id: number;
  userId: number;
  priceId: number;
  amount: number;
  productionDate: Date;
  takenDate: Date;
  description: string;
  isPaid: boolean;
}

export interface RangePaymentInfo {
  from: Date,
  to: Date,
  recordWithPrice: TakingRecordWithPrice[],
  detailInformation: DetailInformation,
  customer: UserModel
}
