import type { MoneyHistoryModel, UserModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asJson, post } from "./fetch-wrapper"

const CUSTOMER_ADD_MONEY = "/customer/add-money"
const CUSTOMER_GET_ALL_USER_MONEY = "/customer/get-all-user-money"


interface ICustomerMoneyApi {
  addMoney: (userId: number, amount: number) => Promise<UserModel>
  getAllUserMoneyHistory: (userId: number) => Promise<Array<MoneyHistoryModel>>
}

const customerMoneyApi: ICustomerMoneyApi = {
  addMoney: async function (userId: number, amount: number): Promise<UserModel> {
    const response = await post(`${API_ENDPOINT}${CUSTOMER_ADD_MONEY}`, {
      userId,
      amount
    });
    return asJson<UserModel>(response)
  },

  getAllUserMoneyHistory: async function (userId: number): Promise<Array<MoneyHistoryModel>> {
    const response = await post(`${API_ENDPOINT}${CUSTOMER_GET_ALL_USER_MONEY}`, { userId });
    return asJson<Array<MoneyHistoryModel>>(response)
  }
}

const customerMoneyCommand: ICustomerMoneyApi = {
  addMoney: function (userId: number, amount: number): Promise<UserModel> {
    throw new Error("Function not implemented.")
  },
  getAllUserMoneyHistory: function (userId: number): Promise<Array<MoneyHistoryModel>> {
    throw new Error("Function not implemented.")
  }
}

export const customerMoney = IS_INSIDE_TAURI ? customerMoneyCommand : customerMoneyApi
