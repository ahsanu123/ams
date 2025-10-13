import type { TakingRecordModel } from "@/api-models"
import { asConstant, asJson, get } from "./fetch-wrapper"

const ADD_NEW_TAKING_RECORD = "/taking-record/add-new-taking-record"
const GET_TAKING_RECORD_BY_DATE = "/taking-record/get-taking-record-by-date"
const GET_TAKING_RECORD_BY_USER_ID = "/taking-record/get-taking-record-by-user-id"
const GET_TAKING_RECORD_BY_USER_ID_AND_MONTH = "/taking-record/get-taking-record-by-user-id-and-date"
const UPSERT_TAKING_RECORD = "/taking-record/upsert-taking-record"

interface ITakingRecordApi {
  addNewTakingRecord: (userId: number, amount: number) => Promise<number>
  getTakingRecordByUserId: (userId: number) => Promise<Array<TakingRecordModel>>
  upsertTakingRecord: (record: TakingRecordModel) => Promise<number>
  getTakingRecordByMonth: (date: Date) => Promise<Array<TakingRecordModel>>
  getTakingRecordByUserIdAndMonth: (userId: number, date: Date) => Promise<Array<TakingRecordModel>>
}


export const takingRecordApi: ITakingRecordApi = {
  addNewTakingRecord: async function (userId: number, amount: number): Promise<number> {
    const response = await get(ADD_NEW_TAKING_RECORD, {
      body: JSON.stringify({
        amount,
        userId
      })
    })

    return asConstant<number>(response)
  },
  getTakingRecordByUserId: async function (userId: number): Promise<Array<TakingRecordModel>> {
    const response = await get(GET_TAKING_RECORD_BY_USER_ID, {
      body: JSON.stringify({
        userId
      })
    })

    return asJson<Array<TakingRecordModel>>(response)
  },
  upsertTakingRecord: async function (record: TakingRecordModel): Promise<number> {
    const response = await get(UPSERT_TAKING_RECORD, {
      body: JSON.stringify({
        record
      })
    })

    return asConstant<number>(response)
  },
  getTakingRecordByMonth: async function (date: Date): Promise<Array<TakingRecordModel>> {
    const response = await get(GET_TAKING_RECORD_BY_DATE, {
      body: JSON.stringify({
        date
      })
    })

    return asJson<Array<TakingRecordModel>>(response)
  },
  getTakingRecordByUserIdAndMonth: async function (userId: number, date: Date): Promise<Array<TakingRecordModel>> {
    const response = await get(GET_TAKING_RECORD_BY_USER_ID_AND_MONTH, {
      body: JSON.stringify({
        date
      })
    })

    return asJson<Array<TakingRecordModel>>(response)
  }
}
