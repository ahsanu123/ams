
export interface PaymentHistoryModel {
  id: number;
  userId: bigint;
  date: Date;
  billAmount: bigint;
  initialMoney: bigint;
  endMoney: bigint;
  addedMoney: bigint;
}
