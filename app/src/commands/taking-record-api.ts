import type { TakingRecordModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asConstant, asJson, del, post } from "./fetch-wrapper"

const ADD_NEW_TAKING_RECORD = "/taking-record/add-new-taking-record"
const ADD_NEW_TAKING_RECORD_BY_DATE = "/taking-record/add-new-taking-record-by-date"
const GET_TAKING_RECORD_BY_DATE = "/taking-record/get-taking-record-by-date"
const GET_TAKING_RECORD_BY_USER_ID = "/taking-record/get-taking-record-by-user-id"
const GET_TAKING_RECORD_BY_USER_ID_AND_MONTH = "/taking-record/get-taking-record-by-user-id-and-date"
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
  deleteTakingRecordById: (id: number) => Promise<number>
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
  deleteTakingRecordById: async function (id: number): Promise<number> {
    const response = await del(`${API_ENDPOINT}${DELETE_TAKING_RECORD_BY_ID}`, { id })
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
  addNewTakingRecord: function (userId: number, amount: number): Promise<number> {
    throw new Error("Function not implemented.")
  },
  getTakingRecordByUserId: function (userId: number): Promise<Array<TakingRecordModel>> {
    throw new Error("Function not implemented.")
  },
  upsertTakingRecord: function (record: TakingRecordModel): Promise<number> {
    throw new Error("Function not implemented.")
  },
  getTakingRecordByMonth: function (date: Date): Promise<Array<TakingRecordModel>> {
    throw new Error("Function not implemented.")
  },
  getTakingRecordByUserIdAndMonth: function (userId: number, date: Date): Promise<Array<TakingRecordModel>> {
    throw new Error("Function not implemented.")
  },
  deleteTakingRecordById: function (id: number): Promise<number> {
    throw new Error("Function not implemented.")
  },
  getTakingRecordByDay: function (date: Date): Promise<Array<TakingRecordModel>> {
    throw new Error("Function not implemented.")
  },
  upsertTakingRecordByDate: function (amount: number, date: Date, userId: number): Promise<number> {
    throw new Error("Function not implemented.")
  },
  addNewTakingRecordByDate: function (userId: number, amount: number, date: Date): Promise<number> {
    throw new Error("Function not implemented.")
  }
}

export const takingRecordCommand = IS_INSIDE_TAURI ? takingRecordTauriCommand : takingRecordApi
