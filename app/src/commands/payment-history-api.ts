import type { PaymentHistoryModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asConstant, asJson, post } from "./fetch-wrapper"

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

const paymentHistoryApi: IPaymentHistoryApi = {
  getPaymentRecord: async function (userId: number): Promise<Array<PaymentHistoryModel>> {
    const response = await post(`${API_ENDPOINT}${GET_PAYMENT_RECORD}`, { userId })

    return asJson<Array<PaymentHistoryModel>>(response)
  },
  getMonthSummary: async function (date: Date): Promise<Array<PaymentHistoryModel>> {
    const response = await post(`${API_ENDPOINT}${GET_MONTH_SUMMARY}`, { date })
    return asJson<Array<PaymentHistoryModel>>(response)
  },
  getPaymentRecordByUserIdAndMonth: async function (userId: number, date: Date): Promise<Array<PaymentHistoryModel>> {
    const response = await post(`${API_ENDPOINT}${GET_PAYMENT_RECORD_BY_USER_ID_AND_MONTH}`, {
      date,
      userId
    })

    return asJson<Array<PaymentHistoryModel>>(response)
  },
  updatePaymentRecord: async function (record: PaymentHistoryModel): Promise<PaymentHistoryModel> {
    const response = await post(`${API_ENDPOINT}${UPDATE_PAYMENT_RECORD}`, { record })

    return asJson<PaymentHistoryModel>(response)
  },
  updateBulkPaymentRecord: async function (records: PaymentHistoryModel, paid: boolean): Promise<number> {
    const response = await post(`${API_ENDPOINT}${UPDATE_BULK_PAYMENT_RECORD}`, {
      records,
      paid
    })

    return asConstant<number>(response)
  }
}

const paymentHistoryTauriCommand: IPaymentHistoryApi = {
  getMonthSummary: function (date: Date): Promise<Array<PaymentHistoryModel>> {
    throw new Error("Function not implemented.")
  },
  getPaymentRecord: function (userId: number): Promise<Array<PaymentHistoryModel>> {
    throw new Error("Function not implemented.")
  },
  getPaymentRecordByUserIdAndMonth: function (userId: number, date: Date): Promise<Array<PaymentHistoryModel>> {
    throw new Error("Function not implemented.")
  },
  updateBulkPaymentRecord: function (records: PaymentHistoryModel, paid: boolean): Promise<number> {
    throw new Error("Function not implemented.")
  },
  updatePaymentRecord: function (record: PaymentHistoryModel): Promise<PaymentHistoryModel> {
    throw new Error("Function not implemented.")
  }
}

export const paymentHistoryCommand = IS_INSIDE_TAURI ? paymentHistoryTauriCommand : paymentHistoryApi 
