import type { UserModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asConstant, asJson, get, post } from "./fetch-wrapper"
import { invoke } from "@tauri-apps/api/core"

const GET_ALL_ACTIVE_USER = "/user-management/get-all-active-user"
const GET_ALL_USER = "/user-management/get-all-user"
const INSERT_NEW_USER = "/user-management/insert-new-user"
const UPSERT_USER = "/user-management/upsert-user"
const GET_BY_USER_ID = "/user-management/get-by-user-id"
const CREATE_NEW_USER = "/user-management/create-new-user"

interface IUserManagementApi {
  createNewUser: (username: string) => Promise<number>
  insertNewUser: (user: UserModel) => Promise<number>
  getAllUser: () => Promise<Array<UserModel>>
  getAllActiveUser: () => Promise<Array<UserModel>>
  upsertUser: (user: UserModel) => Promise<number>
  getById: (userId: number) => Promise<UserModel>
}

const userManagementApi: IUserManagementApi = {
  createNewUser: async function (username: string): Promise<number> {
    const response = await post(`${API_ENDPOINT}${CREATE_NEW_USER}`, { username })
    return asConstant<number>(response)
  },

  insertNewUser: async function (user: UserModel): Promise<number> {
    const response = await post(`${API_ENDPOINT}${INSERT_NEW_USER}`, { user })
    return asConstant<number>(response)
  },

  getAllUser: async function (): Promise<Array<UserModel>> {
    const response = await get(`${API_ENDPOINT}${GET_ALL_USER}`)
    return asJson<Array<UserModel>>(response)
  },

  getAllActiveUser: async function (): Promise<Array<UserModel>> {
    const response = await get(`${API_ENDPOINT}${GET_ALL_ACTIVE_USER}`)
    return asJson<Array<UserModel>>(response)
  },

  upsertUser: async function (user: UserModel): Promise<number> {
    const response = await post(`${API_ENDPOINT}${UPSERT_USER}`, { user })

    return asConstant<number>(response)
  },
  getById: async function (userId: number): Promise<UserModel> {
    const response = await post(`${API_ENDPOINT}${GET_BY_USER_ID}`, { userId })

    return asConstant<UserModel>(response)
  },
}

const userManagementTauriCommand: IUserManagementApi = {
  getAllUser: async function (): Promise<Array<UserModel>> {
    return await invoke('get_all_user')
  },
  getAllActiveUser: async function (): Promise<Array<UserModel>> {
    return await invoke('get_all_active_user')
  },
  insertNewUser: async function (new_user: UserModel): Promise<number> {
    return await invoke('insert_new_user', { new_user })
  },
  upsertUser: async function (user: UserModel): Promise<number> {
    return await invoke('upsert_user', { user })
  },
  getById: async function (id: number): Promise<UserModel> {
    return await invoke('get_by_user_id', { id })
  },
  createNewUser: async function (username: string): Promise<number> {
    return await invoke('create_new_user', { username })

  }
}

export const userManagementCommand = IS_INSIDE_TAURI ? userManagementTauriCommand : userManagementApi
