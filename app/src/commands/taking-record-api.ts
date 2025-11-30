import type { RangePaymentInfo, TakingRecordModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { invoke } from "@tauri-apps/api/core"
import { asConstant, asJson, del, post, transformObjectDates } from "./fetch-wrapper"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

const ADD_NEW_TAKING_RECORD = "/taking-record/add-new-taking-record"
const ADD_NEW_TAKING_RECORD_BY_DATE = "/taking-record/add-new-taking-record-by-date"
const GET_TAKING_RECORD_BY_DATE = "/taking-record/get-taking-record-by-date"
const GET_TAKING_RECORD_BY_USER_ID = "/taking-record/get-taking-record-by-user-id"
const GET_TAKING_RECORD_BY_USER_ID_AND_RANGE_MONTH = "/taking-record/get-taking-record-by-user-id-and-month-range"
const GET_TAKING_RECORD_BY_USER_ID_AND_MONTH = "/taking-record/get-taking-record-by-user-id-and-date"
const GET_TAKING_RECORD_BY_USER_ID_AND_YEAR = "/taking-record/get-taking-record-by-user-id-and-year"
const UPSERT_TAKING_RECORD = "/taking-record/upsert-taking-record"
const DELETE_TAKING_RECORD_BY_ID = "/taking-record/delete-taking-record-by-id"
const GET_TAKING_RECORD_BY_DAY = "/taking-record/get-taking-record-by-day"
const UPSERT_TAKING_RECORD_BY_DATE = "/taking-record/upsert-taking-record-by-date"

interface ITakingRecordApi {
  addNewTakingRecord: (userId: number, amount: number) => Promise<number>
  addNewTakingRecordByDate: (userId: number, amount: number, date: Date) => Promise<number>
  getTakingRecordByUserId: (userId: number) => Promise<Array<TakingRecordModel>>
  upsertTakingRecord: (record: TakingRecordModel) => Promise<number>
  upsertTakingRecordByDate: (amount: number, date: Date, userId: number) => Promise<number>
  getTakingRecordByMonth: (date: Date) => Promise<Array<TakingRecordModel>>
  getTakingRecordByUserIdAndMonth: (userId: number, date: Date) => Promise<Array<TakingRecordModel>>
  getTakingRecordByUserIdAndYear: (userId: number, date: Date) => Promise<Array<TakingRecordModel>>
  getTakingRecordByUserIdAndRangeMonth: (userId: number, from: Date, to: Date) => Promise<RangePaymentInfo>
  deleteTakingRecordById: (takingRecordId: number) => Promise<number>
  getTakingRecordByDay: (date: Date) => Promise<Array<TakingRecordModel>>
}


const takingRecordApi: ITakingRecordApi = {
  addNewTakingRecord: async function (userId: number, amount: number): Promise<number> {
    const response = await post(`${API_ENDPOINT}${ADD_NEW_TAKING_RECORD}`, {
      amount,
      userId
    })

    return asConstant<number>(response)
  },

  addNewTakingRecordByDate: async function (userId: number, amount: number, date: Date): Promise<number> {
    const response = await post(`${API_ENDPOINT}${ADD_NEW_TAKING_RECORD_BY_DATE}`, {
      amount,
      userId,
      date
    })

    return asConstant<number>(response)
  },

  getTakingRecordByUserId: async function (userId: number): Promise<Array<TakingRecordModel>> {
    const response = await post(`${API_ENDPOINT}${GET_TAKING_RECORD_BY_USER_ID}`, { userId })
    return asJson<Array<TakingRecordModel>>(response)
  },

  upsertTakingRecord: async function (record: TakingRecordModel): Promise<number> {
    const response = await post(`${API_ENDPOINT}${UPSERT_TAKING_RECORD}`, { record })
    return asConstant<number>(response)
  },

  getTakingRecordByMonth: async function (date: Date): Promise<Array<TakingRecordModel>> {
    const response = await post(`${API_ENDPOINT}${GET_TAKING_RECORD_BY_DATE}`, { date })
    return asJson<Array<TakingRecordModel>>(response)
  },
  getTakingRecordByUserIdAndMonth: async function (userId: number, date: Date): Promise<Array<TakingRecordModel>> {
    const response = await post(`${API_ENDPOINT}${GET_TAKING_RECORD_BY_USER_ID_AND_MONTH}`, { userId, date })
    return asJson<Array<TakingRecordModel>>(response)
  },
  getTakingRecordByUserIdAndRangeMonth: async function (userId: number, from: Date, to: Date): Promise<RangePaymentInfo> {
    const response = await post(`${API_ENDPOINT}${GET_TAKING_RECORD_BY_USER_ID_AND_RANGE_MONTH}`, { userId, from, to })
    return asJson<RangePaymentInfo>(response)
  },

  getTakingRecordByUserIdAndYear: async function (userId: number, date: Date): Promise<Array<TakingRecordModel>> {
    const response = await post(`${API_ENDPOINT}${GET_TAKING_RECORD_BY_USER_ID_AND_YEAR}`, { userId, date })
    return asJson<Array<TakingRecordModel>>(response)
  },
  deleteTakingRecordById: async function (takingRecordId: number): Promise<number> {
    const response = await del(`${API_ENDPOINT}${DELETE_TAKING_RECORD_BY_ID}`, { takingRecordId })
    return asJson<number>(response)
  },
  getTakingRecordByDay: async function (date: Date): Promise<Array<TakingRecordModel>> {
    const response = await post(`${API_ENDPOINT}${GET_TAKING_RECORD_BY_DAY}`, { date })
    return asJson<Array<TakingRecordModel>>(response)
  },
  upsertTakingRecordByDate: async function (amount: number, date: Date, userId: number): Promise<number> {
    const response = await post(`${API_ENDPOINT}${UPSERT_TAKING_RECORD_BY_DATE}`, {
      amount,
      date,
      userId
    })
    return asJson<number>(response)
  },
}

const takingRecordTauriCommand: ITakingRecordApi = {
  addNewTakingRecord: async function (user_id: number, amount: number): Promise<number> {
    return await invoke('add_new_taking_record', { user_id, amount })
  },
  getTakingRecordByUserId: async function (user_id: number): Promise<Array<TakingRecordModel>> {
    return await invoke('get_taking_record_by_user_id', { user_id })
  },
  upsertTakingRecord: async function (record: TakingRecordModel): Promise<number> {
    return await invoke('upsert_taking_record', transformObjectDates({ record }))
  },
  getTakingRecordByMonth: async function (date: Date): Promise<Array<TakingRecordModel>> {
    return await invoke('get_taking_record_by_month', transformObjectDates({ date }))
  },
  getTakingRecordByUserIdAndMonth: async function (user_id: number, date: Date): Promise<Array<TakingRecordModel>> {
    return await invoke('get_taking_record_by_user_id_and_month', transformObjectDates({ user_id, date }))
  },
  deleteTakingRecordById: async function (record_id: number): Promise<number> {
    return await invoke('delete_taking_record_by_id', { record_id })
  },
  getTakingRecordByDay: async function (date: Date): Promise<Array<TakingRecordModel>> {
    return await invoke('get_taking_record_by_day', transformObjectDates({ date }))
  },
  upsertTakingRecordByDate: async function (amount: number, date: Date, user_id: number): Promise<number> {
    return await invoke('upsert_taking_record_by_date', transformObjectDates({ amount, date, user_id }))
  },
  addNewTakingRecordByDate: async function (user_id: number, amount: number, date: Date): Promise<number> {
    return await invoke('add_new_taking_record_by_date', transformObjectDates({ user_id, amount, date }))
  },
  getTakingRecordByUserIdAndYear: async function (user_id: number, date: Date): Promise<Array<TakingRecordModel>> {
    return await invoke('get_taking_record_by_user_id_and_year', transformObjectDates({ user_id, date }))
  },
  getTakingRecordByUserIdAndRangeMonth: async function (user_id: number, from: Date, to: Date): Promise<RangePaymentInfo> {
    return await invoke('get_taking_record_by_user_id_and_month_range', transformObjectDates({ user_id, from, to }))
  }
}

export const takingRecordCommand = IS_INSIDE_TAURI ? takingRecordTauriCommand : takingRecordApi

export function useTakingRecordCommand() {

  const queryClient = useQueryClient()

  const addNewtakingRecord = (user_id: number, amount: number) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getTakingRecords']
    }),
    mutationFn: () => takingRecordCommand.addNewTakingRecord(user_id, amount)
  })

  const addNewTakingRecordByDate = (userId: number, amount: number, date: Date) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getTakingRecords']
    }),
    mutationFn: () => takingRecordCommand.addNewTakingRecordByDate(userId, amount, date)
  })

  const getTakingRecordByUserId = (userId: number) => useQuery({
    queryKey: ['getTakingRecordByUserId'],
    queryFn: () => takingRecordCommand.getTakingRecordByUserId(userId)
  })

  const upsertTakingRecord = (record: TakingRecordModel) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getTakingRecords'],
    }),
    mutationFn: () => takingRecordCommand.upsertTakingRecord(record)
  })

  const upsertTakingRecordByDate = (amount: number, date: Date, userId: number) => useMutation({

    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getTakingRecords'],
    }),
    mutationFn: () => takingRecordCommand.upsertTakingRecordByDate(amount, date, userId)
  })

  const getTakingRecordByMonth = (date: Date) => useQuery({
    queryKey: ['getTakingRecordByMonth '],
    queryFn: () => takingRecordCommand.getTakingRecordByMonth(date)
  })

  const getTakingRecordByUserIdAndMonth = (userId: number, date: Date) => useQuery({
    queryKey: ['getTakingRecordByUserIdAndMonth'],
    queryFn: () => takingRecordCommand.getTakingRecordByUserIdAndMonth(userId, date)
  })

  const getTakingRecordByUserIdAndYear = (userId: number, date: Date) => useQuery({
    queryKey: ['getTakingRecordByUserIdAndYear'],
    queryFn: () => takingRecordCommand.getTakingRecordByUserIdAndYear(userId, date)
  })

  const getTakingRecordByUserIdAndRangeMonth = (userId: number, from: Date, to: Date) => useQuery({
    queryKey: ['getTakingRecordByUserIdAndRangeMonth'],
    queryFn: () => takingRecordCommand.getTakingRecordByUserIdAndRangeMonth(userId, from, to)
  })

  const deleteTakingRecordById = (takingRecordId: number) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getTakingRecords']
    }),
    mutationFn: () => takingRecordCommand.deleteTakingRecordById(takingRecordId)
  })

  const getTakingRecordByDay = (date: Date) => useQuery({
    queryKey: ['getTakingRecordByDay'],
    queryFn: () => takingRecordCommand.getTakingRecordByDay(date)
  })

  return {
    addNewtakingRecord,
    addNewTakingRecordByDate,
    getTakingRecordByUserId,
    upsertTakingRecord,
    upsertTakingRecordByDate,
    getTakingRecordByMonth,
    getTakingRecordByUserIdAndMonth,
    getTakingRecordByUserIdAndYear,
    getTakingRecordByUserIdAndRangeMonth,
    deleteTakingRecordById,
    getTakingRecordByDay
  }
}

