import type { PaymentHistoryModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asConstant, asJson, post, transformObjectDates } from "./fetch-wrapper"
import { invoke } from "@tauri-apps/api/core"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

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
  getMonthSummary: async function (date: Date): Promise<Array<PaymentHistoryModel>> {
    return await invoke('get_month_summary', transformObjectDates({ date }))
  },
  getPaymentRecord: async function (user_id: number): Promise<Array<PaymentHistoryModel>> {
    return await invoke('get_payment_record_by_user_id', { user_id })
  },
  getPaymentRecordByUserIdAndMonth: async function (user_id: number, date: Date): Promise<Array<PaymentHistoryModel>> {
    return await invoke('get_payment_record_by_user_id_and_month', transformObjectDates({ user_id, date }))
  },
  updateBulkPaymentRecord: async function (records: PaymentHistoryModel, paid: boolean): Promise<number> {
    return await invoke('update_bulk_payment_record', transformObjectDates({ records, paid }))
  },
  updatePaymentRecord: async function (record: PaymentHistoryModel): Promise<PaymentHistoryModel> {
    return await invoke('update_payment_record', transformObjectDates({ record }))
  }
}

export const paymentHistoryCommand = IS_INSIDE_TAURI ? paymentHistoryTauriCommand : paymentHistoryApi

export function usePaymentHistoryCommand() {
  const queryClient = useQueryClient()

  const getMonthSummary = (date: Date) => useQuery({
    queryKey: ['getMonthSummary '],
    queryFn: () => paymentHistoryCommand.getMonthSummary(date)
  })

  const getPaymentRecord = (userId: number) => useQuery({
    queryKey: ['getPaymentRecord '],
    queryFn: () => paymentHistoryCommand.getPaymentRecord(userId)
  })

  const getPaymentRecordByUserIdAndMonth = (userId: number | undefined, date: Date | undefined) => ({
    queryKey: ['getPaymentRecordByUserIdAndMonth', userId, date],
    queryFn: () => paymentHistoryCommand.getPaymentRecordByUserIdAndMonth(userId!, date!),
    enabled: !!userId && !!date
  })

  const updateBulkPaymentRecord = (records: PaymentHistoryModel, paid: boolean) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getPaymentRecord']
    }),
    mutationFn: () => paymentHistoryCommand.updateBulkPaymentRecord(records, paid)
  })

  const updatePaymentRecord = (record: PaymentHistoryModel) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getPaymentRecord']
    }),
    mutationFn: () => paymentHistoryCommand.updatePaymentRecord(record)
  })

  return {
    getMonthSummary,
    getPaymentRecord,
    getPaymentRecordByUserIdAndMonth,
    updateBulkPaymentRecord,
    updatePaymentRecord,
  }
}

