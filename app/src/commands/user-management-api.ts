// TODO: add return type
interface IUserManagementApi {
  insertNewUser: (/*request_model::InsertNewUser*/) => void
  getAllUser: () => void
  GetAllActiveUser: () => void
  UpsertUser: (/*request_model::UpsertUser*/) => void
}


export const userManagementApi: IUserManagementApi = {
  insertNewUser: function (): void {
    throw new Error("Function not implemented.")
  },
  getAllUser: function (): void {
    throw new Error("Function not implemented.")
  },
  GetAllActiveUser: function (): void {
    throw new Error("Function not implemented.")
  },
  UpsertUser: function (): void {
    throw new Error("Function not implemented.")
  }
}
