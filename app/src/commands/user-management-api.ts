import type { UserModel } from "@/api-models"
import { asConstant, asJson, get, post } from "./fetch-wrapper"

const GET_ALL_ACTIVE_USER = "/user-management/get-all-active-user"
const GET_ALL_USER = "/user-management/get-all-user"
const INSERT_NEW_USER = "/user-management/insert-new-user"
const UPSERT_USER = "/user-management/upsert-user"

// TODO: add return type
interface IUserManagementApi {
  getAllUser: () => Promise<Array<UserModel>>
  GetAllActiveUser: () => Promise<Array<UserModel>>
  insertNewUser: (user: UserModel) => Promise<number>
  UpsertUser: (user: UserModel) => Promise<number>
}


export const userManagementApi: IUserManagementApi = {
  getAllUser: async function (): Promise<Array<UserModel>> {
    const response = await get(GET_ALL_USER)

    return asJson<Array<UserModel>>(response)
  },
  GetAllActiveUser: async function (): Promise<Array<UserModel>> {
    const response = await get(GET_ALL_ACTIVE_USER)

    return asJson<Array<UserModel>>(response)
  },
  insertNewUser: async function (user: UserModel): Promise<number> {
    const response = await post(INSERT_NEW_USER, {
      body: JSON.stringify({
        user
      })
    })

    return asConstant<number>(response)
  },
  UpsertUser: async function (user: UserModel): Promise<number> {
    const response = await post(UPSERT_USER, {
      body: JSON.stringify({
        user
      })
    })

    return asConstant<number>(response)
  }
}
