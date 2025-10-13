import type { MoneyHistoryModel, UserModel } from "@/api-models"
import { asJson, post } from "./fetch-wrapper"
import { API_ENDPOINT } from "@/constants"

const CUSTOMER_ADD_MONEY = "/customer/add-money"
const CUSTOMER_GET_ALL_USER_MONEY = "/customer/get-all-user-money"


interface ICustomerMoneyApi {
  addMoney: (userId: number, amount: number) => Promise<UserModel>
  getAllUserMoneyHistory: (userId: number) => Promise<Array<MoneyHistoryModel>>
}

export const customerMoneyApi: ICustomerMoneyApi = {
  addMoney: async function (userId: number, amount: number): Promise<UserModel> {
    const response = await post(`${API_ENDPOINT}${CUSTOMER_ADD_MONEY}`, {
      body: JSON.stringify({
        userId,
        amount
      })
    });

    return asJson<UserModel>(response)
  },

  getAllUserMoneyHistory: async function (userId: number): Promise<Array<MoneyHistoryModel>> {
    const response = await post(`${API_ENDPOINT}${CUSTOMER_GET_ALL_USER_MONEY}`, {
      body: JSON.stringify({
        userId,
      })
    });

    return asJson<Array<MoneyHistoryModel>>(response)
  }
}


