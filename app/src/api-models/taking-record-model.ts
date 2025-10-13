export interface TakingRecordModel {
  id: number;
  userId: bigint;
  priceId: bigint;
  amount: bigint;
  productionDate: Date;
  takenDate: Date;
  description: string;
  isPaid: boolean;
}
