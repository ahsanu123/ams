// // dreg_price_endpoint
// crate::endpoints::dreg_price_endpoint::get_latest_dreg_price,
// crate::endpoints::dreg_price_endpoint::update_dreg_price,
// crate::endpoints::dreg_price_endpoint::get_all_dreg_price,


// TODO: add return type
interface IDregPriceApi {
  getLatestDregPrice: () => void
  updateDregPrice: (newPrice: number) => void
  getAllDregPrice: () => void
}


export const dregPriceApi: IDregPriceApi = {
  getLatestDregPrice: function (): void {
    throw new Error("Function not implemented.")
  },
  updateDregPrice: function (newPrice: number): void {
    throw new Error("Function not implemented.")
  },
  getAllDregPrice: function (): void {
    throw new Error("Function not implemented.")
  }
}
