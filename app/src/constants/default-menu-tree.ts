import { AdminMenus, UserManagementMenu, type MenuTree } from "@/model";
import CreateNewUserPage from "@/page/CreateNewUserPage";
import DeleteUserPage from "@/page/DeleteUserPage";
import EditUserPage from "@/page/EditUserPage";
import ListUserPage from "@/page/ListUserPage";
import UserManagementPage from "@/page/UserManagementPage";
import type { JSX } from "react";


const userManagementMenu: MenuTree = {
  key: AdminMenus.UserManagement,
  isRoot: true,
  component: UserManagementPage,
  children: [
    {
      key: UserManagementMenu.CreateNewUser,
      component: CreateNewUserPage
    },
    {
      key: UserManagementMenu.EditUser,
      component: EditUserPage
    },
    {
      key: UserManagementMenu.ListUser,
      component: ListUserPage
    },
    {
      key: UserManagementMenu.DeleteUser,
      component: DeleteUserPage
    }
  ]
}

const editRecordMenu: MenuTree = {
  key: AdminMenus.EditRecord,
  isRoot: true,
  component: function (): JSX.Element {
    throw new Error("Function not implemented.");
  }
}

const paymentMenu: MenuTree = {
  key: AdminMenus.PaymentMenu,
  isRoot: true,
  component: function (): JSX.Element {
    throw new Error("Function not implemented.");
  }
}

const printDataMenu: MenuTree = {
  key: AdminMenus.PrintData,
  isRoot: true,
  component: function (): JSX.Element {
    throw new Error("Function not implemented.");
  }
}

const calendarMenu: MenuTree = {
  key: AdminMenus.Calendar,
  isRoot: true,
  component: function (): JSX.Element {
    throw new Error("Function not implemented.");
  },
  children: [
    {
      key: UserManagementMenu.CreateNewUser,
      component: function (): JSX.Element {
        throw new Error("Function not implemented.");
      }
    },
    {
      key: UserManagementMenu.EditUser,
      component: function (): JSX.Element {
        throw new Error("Function not implemented.");
      }
    },
    {
      key: UserManagementMenu.ListUser,
      component: function (): JSX.Element {
        throw new Error("Function not implemented.");
      }
    },
    {
      key: UserManagementMenu.DeleteUser,
      component: function (): JSX.Element {
        throw new Error("Function not implemented.");
      }
    }
  ]
}
export const defaultMenuTree: MenuTree[] = [
  userManagementMenu,
  editRecordMenu,
  paymentMenu,
  printDataMenu,
  calendarMenu
]
