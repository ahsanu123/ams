export interface MoneyHistoryModel {
  id: number;
  userId: number;
  date: Date;
  moneyAmount: bigint;
  description: string;
}
