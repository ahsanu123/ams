import type { UserModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asConstant, asJson, get, post } from "./fetch-wrapper"

const GET_ALL_ACTIVE_USER = "/user-management/get-all-active-user"
const GET_ALL_USER = "/user-management/get-all-user"
const INSERT_NEW_USER = "/user-management/insert-new-user"
const UPSERT_USER = "/user-management/upsert-user"

// TODO: add return type
interface IUserManagementApi {
  getAllUser: () => Promise<Array<UserModel>>
  getAllActiveUser: () => Promise<Array<UserModel>>
  insertNewUser: (user: UserModel) => Promise<number>
  UpsertUser: (user: UserModel) => Promise<number>
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

  UpsertUser: async function (user: UserModel): Promise<number> {
    const response = await post(`${API_ENDPOINT}${UPSERT_USER}`, { user })

    return asConstant<number>(response)
  }
}

const userManagementCommand: IUserManagementApi = {
  getAllUser: function (): Promise<Array<UserModel>> {
    throw new Error("Function not implemented.")
  },
  getAllActiveUser: function (): Promise<Array<UserModel>> {
    throw new Error("Function not implemented.")
  },
  insertNewUser: function (user: UserModel): Promise<number> {
    throw new Error("Function not implemented.")
  },
  UpsertUser: function (user: UserModel): Promise<number> {
    throw new Error("Function not implemented.")
  }
}

export const userManagement = IS_INSIDE_TAURI ? userManagementCommand : userManagementApi
