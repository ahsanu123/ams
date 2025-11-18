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
