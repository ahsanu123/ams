import type { DregPriceModel } from "@/api-models"
import { asJson, get, post } from "./fetch-wrapper"
import { API_ENDPOINT } from "@/constants"

const GET_LATEST_DREG_PRICE = "/dreg-price/get-latest-dreg-price"
const GET_ALL_DREG_PRICE = "/dreg-price/get-all-price"
const UPDATE_DREG_PRICE = "/dreg-price/update-dreg-price"

interface IDregPriceApi {
  getLatestDregPrice: () => Promise<DregPriceModel>
  updateDregPrice: (newPrice: number) => Promise<DregPriceModel>
  getAllDregPrice: () => Promise<Array<DregPriceModel>>
}

export const dregPriceApi: IDregPriceApi = {
  getLatestDregPrice: async function (): Promise<DregPriceModel> {
    const response = await get(`${API_ENDPOINT}${GET_LATEST_DREG_PRICE}`);

    return asJson<DregPriceModel>(response)
  },
  updateDregPrice: async function (newPrice: number): Promise<DregPriceModel> {
    const result = await post(`${API_ENDPOINT}${UPDATE_DREG_PRICE}`, {
      body: JSON.stringify({
        newPrice
      })
    })

    return asJson<DregPriceModel>(result)
  },
  getAllDregPrice: async function (): Promise<Array<DregPriceModel>> {
    const response = await get(`${API_ENDPOINT}${GET_ALL_DREG_PRICE}`);

    return asJson<Array<DregPriceModel>>(response)
  }
}
