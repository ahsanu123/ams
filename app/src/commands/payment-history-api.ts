// //payment_history_endpoint
// crate::endpoints::payment_history_endpoint::get_payment_record_by_user_id,
// crate::endpoints::payment_history_endpoint::get_month_summary,
// crate::endpoints::payment_history_endpoint::get_payment_record_by_user_id_and_month,
// crate::endpoints::payment_history_endpoint::get_month_summary_by_user_id,
// crate::endpoints::payment_history_endpoint::update_payment_record,
// crate::endpoints::payment_history_endpoint::update_bulk_payment_record,


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
