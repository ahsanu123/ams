// crate::endpoints::customer_endpoints::add_money,
// crate::endpoints::customer_endpoints::get_all_user_money_history,

// TODO: add return type
interface ICustomerApi {
  addMoney: (userId: number, amount: number) => void
  getAllUserMoneyHistory: (userId: number) => void
}


export const customerApi: ICustomerApi = {
  addMoney: function (userId: number, amount: number): void {
    throw new Error("Function not implemented.")
  },
  getAllUserMoneyHistory: function (userId: number): void {
    throw new Error("Function not implemented.")
  }
}
