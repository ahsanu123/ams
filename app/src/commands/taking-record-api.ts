import type { TakingRecordModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asConstant, asJson, del, post, transformObjectDates } from "./fetch-wrapper"
import { invoke } from "@tauri-apps/api/core"

const ADD_NEW_TAKING_RECORD = "/taking-record/add-new-taking-record"
const ADD_NEW_TAKING_RECORD_BY_DATE = "/taking-record/add-new-taking-record-by-date"
const GET_TAKING_RECORD_BY_DATE = "/taking-record/get-taking-record-by-date"
const GET_TAKING_RECORD_BY_USER_ID = "/taking-record/get-taking-record-by-user-id"
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
  }
}

export const takingRecordCommand = IS_INSIDE_TAURI ? takingRecordTauriCommand : takingRecordApi
