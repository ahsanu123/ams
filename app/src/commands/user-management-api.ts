import type { UserModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asConstant, asJson, get, post } from "./fetch-wrapper"

const GET_ALL_ACTIVE_USER = "/user-management/get-all-active-user"
const GET_ALL_USER = "/user-management/get-all-user"
const INSERT_NEW_USER = "/user-management/insert-new-user"
const UPSERT_USER = "/user-management/upsert-user"
const GET_BY_USER_ID = "/user-management/get-by-user-id"

// TODO: add return type
interface IUserManagementApi {
  getAllUser: () => Promise<Array<UserModel>>
  getAllActiveUser: () => Promise<Array<UserModel>>
  insertNewUser: (user: UserModel) => Promise<number>
  upsertUser: (user: UserModel) => Promise<number>
  getById: (userId: number) => Promise<UserModel>
}


const userManagementApi: IUserManagementApi = {
  getAllUser: async function (): Promise<Array<UserModel>> {
    const response = await get(`${API_ENDPOINT}${GET_ALL_USER}`)
    return asJson<Array<UserModel>>(response)
  },

  getAllActiveUser: async function (): Promise<Array<UserModel>> {
    const response = await get(`${API_ENDPOINT}${GET_ALL_ACTIVE_USER}`)
    return asJson<Array<UserModel>>(response)
  },

  insertNewUser: async function (user: UserModel): Promise<number> {
    const response = await post(`${API_ENDPOINT}${INSERT_NEW_USER}`, { user })
    return asConstant<number>(response)
  },

  upsertUser: async function (user: UserModel): Promise<number> {
    const response = await post(`${API_ENDPOINT}${UPSERT_USER}`, { user })

    return asConstant<number>(response)
  },
  getById: async function (userId: number): Promise<UserModel> {
    const response = await post(`${API_ENDPOINT}${GET_BY_USER_ID}`, { userId })

    return asConstant<UserModel>(response)
  }
}

const userManagementTauriCommand: IUserManagementApi = {
  getAllUser: function (): Promise<Array<UserModel>> {
    throw new Error("Function not implemented.")
  },
  getAllActiveUser: function (): Promise<Array<UserModel>> {
    throw new Error("Function not implemented.")
  },
  insertNewUser: function (user: UserModel): Promise<number> {
    throw new Error("Function not implemented.")
  },
  upsertUser: function (user: UserModel): Promise<number> {
    throw new Error("Function not implemented.")
  },
  getById: function (userId: number): Promise<UserModel> {
    throw new Error("Function not implemented.")
  }
}

export const userManagementCommand = IS_INSIDE_TAURI ? userManagementTauriCommand : userManagementApi
