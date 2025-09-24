import type { FlatMenuTree } from "@/model";
import EditPickingRecordPage from "@/page/edit-picking-record-pages/EditPickingRecordPage";
import CreateNewUserPage from "@/page/user-management-pages/CreateNewUserPage";
import EditUserPage from "@/page/user-management-pages/EditUserPage";
import ListUserPage from "@/page/user-management-pages/ListUserPage";

export enum UserManagementPath {
  Path = "UserManagementPath",
  UpdateUser = "UpdateUser",
  ListAllUser = "ListAllUser",
  CreateOrEdit = "CreateOrEdit"
}

export enum TakingRecordPath {
  Path = "TakingRecordPath ",
  EditTakingRecord = "EditTakingRecord"
}

const userManagementMenu: FlatMenuTree[] = [
  {
    path: `${UserManagementPath.Path}`,
    isRoot: true,
    groupName: `${UserManagementPath.Path}`,
    title: "User Management"
  },
  {
    path: `${UserManagementPath.Path}-${UserManagementPath.UpdateUser}`,
    isRoot: false,
    groupName: `${UserManagementPath.Path}`,
    title: "Update User Page",
    component: <EditUserPage />
  },
  {
    path: `${UserManagementPath.Path}-${UserManagementPath.ListAllUser}`,
    isRoot: false,
    groupName: `${UserManagementPath.Path}`,
    title: "List All User Page",
    component: <ListUserPage />
  },
  {
    path: `${UserManagementPath.Path}-${UserManagementPath.CreateOrEdit}`,
    isRoot: false,
    groupName: `${UserManagementPath.Path}`,
    title: "Create Or Edit User Page",
    component: <CreateNewUserPage />
  },
]

const takingRecordMenu: FlatMenuTree[] = [
  {
    path: `${TakingRecordPath.Path}`,
    isRoot: true,
    groupName: `${TakingRecordPath.Path}`,
    title: "Taking Record"
  },
  {
    path: `${TakingRecordPath.Path}-${TakingRecordPath.EditTakingRecord}`,
    isRoot: false,
    groupName: `${TakingRecordPath.Path}`,
    title: "Edit Taking Record",
    component: <EditPickingRecordPage />
  },
  {
    path: `${TakingRecordPath.Path}-${TakingRecordPath.EditTakingRecord}`,
    isRoot: false,
    groupName: `${TakingRecordPath.Path}`,
    title: "Edit Taking Record",
    component: <EditPickingRecordPage />
  },
]


export const defaultFlatMenuTree: FlatMenuTree[] = [
  ...userManagementMenu,
  ...takingRecordMenu
]
