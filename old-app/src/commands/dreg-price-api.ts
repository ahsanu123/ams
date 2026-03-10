import type { DregPriceModel } from "@/api-models"
import { API_ENDPOINT, IS_INSIDE_TAURI } from "@/constants"
import { asJson, get, post } from "./fetch-wrapper"
import { invoke } from "@tauri-apps/api/core"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

const GET_LATEST_DREG_PRICE = "/dreg-price/get-latest-dreg-price"
const GET_ALL_DREG_PRICE = "/dreg-price/get-all-price"
const UPDATE_DREG_PRICE = "/dreg-price/update-dreg-price"

interface IDregPriceApi {
  getLatestDregPrice: () => Promise<DregPriceModel>
  updateDregPrice: (newPrice: number) => Promise<DregPriceModel>
  getAllDregPrice: () => Promise<Array<DregPriceModel>>
}

const dregPriceApi: IDregPriceApi = {
  getLatestDregPrice: async function (): Promise<DregPriceModel> {
    const response = await get(`${API_ENDPOINT}${GET_LATEST_DREG_PRICE}`);
    return asJson<DregPriceModel>(response)
  },
  updateDregPrice: async function (newPrice: number): Promise<DregPriceModel> {
    const result = await post(`${API_ENDPOINT}${UPDATE_DREG_PRICE}`, { newPrice })
    return asJson<DregPriceModel>(result)
  },
  getAllDregPrice: async function (): Promise<Array<DregPriceModel>> {
    const response = await get(`${API_ENDPOINT}${GET_ALL_DREG_PRICE}`);
    return asJson<Array<DregPriceModel>>(response)
  }
}

const dregPriceTauriCommand: IDregPriceApi = {
  getLatestDregPrice: async function (): Promise<DregPriceModel> {
    return await invoke('get_latest_dreg_price')
  },
  updateDregPrice: async function (new_price: number): Promise<DregPriceModel> {
    return await invoke('update_dreg_price', { newPrice: new_price })
  },
  getAllDregPrice: async function (): Promise<Array<DregPriceModel>> {
    return await invoke('get_all_dreg_price')
  }
}

export const dregPriceCommand = IS_INSIDE_TAURI ? dregPriceTauriCommand : dregPriceApi

export function useDregPriceCommand() {

  const queryClient = useQueryClient()

  const getLatestDregPrice = useQuery({
    queryKey: ['getLatestDregPrice'],
    queryFn: dregPriceCommand.getLatestDregPrice
  })

  const getAllDregPrice = () => ({
    queryKey: ['getAllDregPrice'],
    queryFn: dregPriceCommand.getAllDregPrice
  })
  const updateDregPrice = (newPrice: number) => useMutation({
    onSuccess: () => queryClient.invalidateQueries({
      queryKey: ['getAllDregPrice', 'getLatestDregPrice']
    }),
    mutationFn: () => dregPriceCommand.updateDregPrice(newPrice)
  })

  return {
    getLatestDregPrice,
    updateDregPrice,
    getAllDregPrice,
  }
}

