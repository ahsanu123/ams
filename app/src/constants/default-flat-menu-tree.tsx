import type { FlatMenuTree } from "@/model";
import EditUserPage from "@/page/user-management-pages/EditUserPage";

export enum UserManagementPath {
  Path = "UserManagementPath ",
  UpdateUser = "UpdateUser"
}

export const defaultFlatMenuTree: FlatMenuTree[] = [
  {
    path: `${UserManagementPath.Path}`,
    title: "User Management"
  },
  {
    path: `${UserManagementPath.Path}-${UserManagementPath.UpdateUser}`,
    title: "Update User Page",
    component: <EditUserPage />
  }
]
