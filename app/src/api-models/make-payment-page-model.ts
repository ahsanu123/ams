import type { DregPriceModel } from "./dreg-price-model";
import type { TakingRecordModel } from "./taking-record-model";
import type { UserModel } from "./user-model";

export interface DetailInformation {
  totalBillForCurrentMonth: number;
  takingCountForCurrentMonth: number;
}

export interface TakingRecordWithPrice {
  takingRecord: TakingRecordModel;
  price: DregPriceModel;
}

export interface MakePaymentPageModel {
  takingRecords: TakingRecordWithPrice[];
  detailInformation: DetailInformation;
  customers: UserModel[];
}
