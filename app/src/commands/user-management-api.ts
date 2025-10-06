// // user_management_enpoint
// crate::endpoints::user_management_enpoint::insert_new_user,
// crate::endpoints::user_management_enpoint::get_all_user,
// crate::endpoints::user_management_enpoint::get_all_active_user,
// crate::endpoints::user_management_enpoint::upsert_user,



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
