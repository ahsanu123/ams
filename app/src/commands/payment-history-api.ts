// TODO: add return type
interface IPaymentHistoryApi {
  getPaymentRecordByUserId: (userId: number) => void
  getMonthSummary: (date: Date) => void
  getPaymentRecordByUserIdAndMonth: (userId: number, date: Date) => void
  getMonthSummaryByUserId: (userId: number, date: Date) => void
  updatePaymentRecord: (/*request_model::UpdatePaymentRecord*/) => void
  updateBulkPaymentRecord: (/*request_model::UpdateBulkPaymentRecord*/) => void
}


export const paymentHistoryApi: IPaymentHistoryApi = {
  getPaymentRecordByUserId: function (userId: number): void {
    throw new Error("Function not implemented.")
  },
  getMonthSummary: function (date: Date): void {
    throw new Error("Function not implemented.")
  },
  getPaymentRecordByUserIdAndMonth: function (userId: number, date: Date): void {
    throw new Error("Function not implemented.")
  },
  getMonthSummaryByUserId: function (userId: number, date: Date): void {
    throw new Error("Function not implemented.")
  },
  updatePaymentRecord: function (): void {
    throw new Error("Function not implemented.")
  },
  updateBulkPaymentRecord: function (): void {
    throw new Error("Function not implemented.")
  }
}
