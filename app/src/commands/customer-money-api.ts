import type { MoneyHistoryModel, UserModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asJson, del, post } from "./fetch-wrapper"
import { invoke } from '@tauri-apps/api/core'
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

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
  addMoney: async function (user_id: number, amount: number): Promise<UserModel> {
    return await invoke('add_money', { user_id, amount })
  },
  getAllUserMoneyHistory: async function (user_id: number): Promise<Array<MoneyHistoryModel>> {
    return await invoke('get_all_user_money_history', { user_id })
  },
  deleteUser: async function (user_id: number): Promise<number> {
    return await invoke('delete_customer', { user_id })
  }
}

export const customerMoneyCommand = IS_INSIDE_TAURI ? customerMoneyTauriCommand : customerMoneyApi

export function useCustomerMoneyCommand() {
  const queryClient = useQueryClient()

  const addMoney = (userId: number, amount: number) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getAllUserMoneyHistory ']
    }),
    mutationFn: () => customerMoneyCommand.addMoney(userId, amount)
  })

  const getAllUserMoneyHistory = (userId: number) => useQuery({
    queryKey: ['getAllUserMoneyHistory'],
    queryFn: () => customerMoneyTauriCommand.getAllUserMoneyHistory(userId)
  })

  const deleteUser = (userId: number) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getAllUserMoneyHistory'],
    }),
    mutationFn: () => customerMoneyTauriCommand.deleteUser(userId)
  })

  return {
    addMoney,
    getAllUserMoneyHistory,
    deleteUser,
  }

}

