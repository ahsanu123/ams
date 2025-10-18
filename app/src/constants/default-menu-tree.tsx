import { AdminMenus, UserManagementMenu, type MenuTree } from "@/model";
import EditPickingRecordPage from "@/page/edit-picking-record-pages/EditPickingRecordPage";
import MakePaymentPage from "@/page/make-payment-pages/MakePaymentPages";
import PrintDataPage from "@/page/print-data-pages/PrintDataPages";
import CreateNewUserPage from "@/page/user-management-pages/CreateNewUserPage";
import DeleteUserPage from "@/page/user-management-pages/DeleteUserPage";
import EditUserPage from "@/page/user-management-pages/EditUserPage";
import ListUserPage from "@/page/user-management-pages/ListUserPage";
import UserManagementPage from "@/page/user-management-pages/UserManagementPage";
import type { JSX } from "react";

const userManagementMenu: MenuTree = {
  key: AdminMenus.UserManagement,
  isRoot: true,
  component: <UserManagementPage />,
  children: [
    {
      key: UserManagementMenu.CreateNewUser,
      component: <CreateNewUserPage />
    },
    {
      key: UserManagementMenu.EditUser,
      component: <EditUserPage />
    },
    // {
    //   key: UserManagementMenu.ListUser,
    //   component: ListUserPage
    // },
    {
      key: UserManagementMenu.DeleteUser,
      component: <DeleteUserPage />
    }
  ]
}

const editPickingRecordMenu: MenuTree = {
  key: AdminMenus.EditRecord,
  isRoot: true,
  showSubBottomMenu: true,
  component: <EditPickingRecordPage />,
}

const paymentMenu: MenuTree = {
  key: AdminMenus.PaymentMenu,
  isRoot: true,
  component: <MakePaymentPage />
}

const printDataMenu: MenuTree = {
  key: AdminMenus.PrintData,
  isRoot: true,
  component: <PrintDataPage />,
}

export const defaultMenuTree: MenuTree[] = [
  userManagementMenu,
  editPickingRecordMenu,
  paymentMenu,
  printDataMenu,
]
