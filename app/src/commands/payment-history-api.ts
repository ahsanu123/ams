import type { PaymentHistoryModel } from "@/api-models"
import { API_ENDPOINT } from "@/constants"
import { asJson, get } from "./fetch-wrapper"

const GET_MONTH_SUMMARY = "/payment/get-month-summary"
const GET_PAYMENT_RECORD = "/payment/get-payment-record"
const GET_PAYMENT_RECORD_BY_USER_ID_AND_MONTH = "/payment/get-payment-record-by-user-id-and-month"
const UPDATE_BULK_PAYMENT_RECORD = "/payment/update-bulk-payment-record"
const UPDATE_PAYMENT_RECORD = "/payment/update-payment-record"

interface IPaymentHistoryApi {
  getMonthSummary: (date: Date) => Promise<Array<PaymentHistoryModel>>
  getPaymentRecord: (userId: number) => Promise<Array<PaymentHistoryModel>>
  getPaymentRecordByUserIdAndMonth: (userId: number, date: Date) => Promise<Array<PaymentHistoryModel>>
  updateBulkPaymentRecord: (records: PaymentHistoryModel, paid: boolean) => Promise<number>
  updatePaymentRecord: (record: PaymentHistoryModel) => Promise<PaymentHistoryModel>
}

export const paymentHistoryApi: IPaymentHistoryApi = {
  getPaymentRecord: async function (userId: number): Promise<Array<PaymentHistoryModel>> {
    const response = await get(`${API_ENDPOINT}${GET_PAYMENT_RECORD}`, {
      body: JSON.stringify({
        userId
      })
    })

    return asJson<Array<PaymentHistoryModel>>(response)
  },
  getMonthSummary: async function (date: Date): Promise<Array<PaymentHistoryModel>> {
    const response = await get(`${API_ENDPOINT}${GET_MONTH_SUMMARY}`, {
      body: JSON.stringify({
        date
      })
    })
    return asJson<Array<PaymentHistoryModel>>(response)
  },
  getPaymentRecordByUserIdAndMonth: async function (userId: number, date: Date): Promise<Array<PaymentHistoryModel>> {
    const response = await get(`${API_ENDPOINT}${GET_PAYMENT_RECORD_BY_USER_ID_AND_MONTH}`, {
      body: JSON.stringify({
        date,
        userId
      })
    })

    return asJson<Array<PaymentHistoryModel>>(response)
  },
  updatePaymentRecord: async function (record: PaymentHistoryModel): Promise<PaymentHistoryModel> {
    const response = await get(`${API_ENDPOINT}${UPDATE_PAYMENT_RECORD}`, {
      body: JSON.stringify({
        record
      })
    })

    return asJson<PaymentHistoryModel>(response)
  },
  updateBulkPaymentRecord: async function (records: PaymentHistoryModel, paid: boolean): Promise<number> {
    const response = await get(`${API_ENDPOINT}${UPDATE_BULK_PAYMENT_RECORD}`, {
      body: JSON.stringify({
        records,
        paid
      })
    })

    console.log(response)
    return Promise.resolve(1)
  }
}
