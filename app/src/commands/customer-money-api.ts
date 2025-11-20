import type { MoneyHistoryModel, UserModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asJson, del, post } from "./fetch-wrapper"

const CUSTOMER_ADD_MONEY = "/customer/add-money"
const CUSTOMER_GET_ALL_USER_MONEY = "/customer/get-all-user-money"
const CUSTOMER_DELETE = "/customer/delete"


interface ICustomerMoneyApi {
  addMoney: (userId: number, amount: number) => Promise<UserModel>
  getAllUserMoneyHistory: (userId: number) => Promise<Array<MoneyHistoryModel>>
  deleteUser: (userId: number) => Promise<number>
}

const customerMoneyApi: ICustomerMoneyApi = {
  addMoney: async function (userId: number, amount: number): Promise<UserModel> {
    const response = await post(`${API_ENDPOINT}${CUSTOMER_ADD_MONEY}`, {
      userId,
      amount
    })
    return asJson<UserModel>(response)
  },

  getAllUserMoneyHistory: async function (userId: number): Promise<Array<MoneyHistoryModel>> {
    const response = await post(`${API_ENDPOINT}${CUSTOMER_GET_ALL_USER_MONEY}`, { userId })
    return asJson<Array<MoneyHistoryModel>>(response)
  },

  deleteUser: async function (userId: number): Promise<number> {
    const response = await del(`${API_ENDPOINT}${CUSTOMER_DELETE}`, {
      userId,
    })
    return asJson<number>(response)
  }
}

const customerMoneyTauriCommand: ICustomerMoneyApi = {
  addMoney: function (userId: number, amount: number): Promise<UserModel> {
    throw new Error("Function not implemented.")
  },
  getAllUserMoneyHistory: function (userId: number): Promise<Array<MoneyHistoryModel>> {
    throw new Error("Function not implemented.")
  },
  deleteUser: function (userId: number): Promise<number> {
    throw new Error("Function not implemented.")
  }
}

export const customerMoneyCommand = IS_INSIDE_TAURI ? customerMoneyTauriCommand : customerMoneyApi
