import type { MakePaymentPageModel, MoneyHistoryModel, UserModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asJson, post } from "./fetch-wrapper"
import { invoke } from "@tauri-apps/api/core"

const GET_PAGE_MODEL = "/make-payment-page/get-page-model"
const MAKE_PAYMENT = "/make-payment-page/make-payment"

interface IMakePaymentCommand {
  getPageModel: (userId: number, date: Date) => Promise<MakePaymentPageModel>
  makePayment: (userId: number, date: Date) => Promise<MakePaymentPageModel>
}

const makePaymentApi: IMakePaymentCommand = {
  getPageModel: async function (userId: number, date: Date): Promise<MakePaymentPageModel> {
    const response = await post(`${API_ENDPOINT}${GET_PAGE_MODEL}`, {
      userId,
      date,
    })

    return asJson<MakePaymentPageModel>(response)
  },

  makePayment: async function (userId: number, date: Date): Promise<MakePaymentPageModel> {
    const response = await post(`${API_ENDPOINT}${MAKE_PAYMENT}`, {
      userId,
      date,
    })

    return asJson<MakePaymentPageModel>(response)
  }
}

const makePaymentTauriCommand: IMakePaymentCommand = {
  getPageModel: async function (user_id: number, date: Date): Promise<MakePaymentPageModel> {
    return await invoke('payment_page_get_page_model', { user_id, date })
  },
  makePayment: async function (user_id: number, date: Date): Promise<MakePaymentPageModel> {
    return await invoke('payment_page_make_payment', { user_id, date })
  }
}

export const makePaymentCommand = IS_INSIDE_TAURI ? makePaymentTauriCommand : makePaymentApi

