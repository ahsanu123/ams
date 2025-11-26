import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asJson, post, transformObjectDates } from "./fetch-wrapper"
import { invoke } from "@tauri-apps/api/core"

const GET_PAGE_MODEL = "/make-payment-page/get-page-model"
const MAKE_PAYMENT = "/make-payment-page/make-payment"

interface IPathInfoCommand {
  getRootPath: () => Promise<string>
}

const pathInfoApi: IPathInfoCommand = {
  getRootPath: function (): Promise<string> {
    throw new Error("Function not implemented.")
  }
}

const pathInfoTauriCommand: IPathInfoCommand = {
  getRootPath: function (): Promise<string> {
    throw new Error("Function not implemented.")
  }
}

export const pathInfoCommand = IS_INSIDE_TAURI ? pathInfoTauriCommand : pathInfoApi
