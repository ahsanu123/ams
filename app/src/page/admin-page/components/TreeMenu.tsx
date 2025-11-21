import { AdminRoutes } from '@/routes';
import { Button, IconButton, Stack, Text } from '@chakra-ui/react';
import { useEffect, type JSX } from 'react';
import { AiFillApi, AiFillCaretLeft, AiFillCreditCard, AiFillDatabase, AiFillDropboxSquare, AiFillEdit, AiFillInfoCircle, AiOutlineDollar, AiOutlineShoppingCart, AiOutlineUser, AiOutlineUserAdd, AiOutlineUserSwitch } from "react-icons/ai";
import { useAdminPageStore } from '../admin-page-state';
import './TreeMenu.css';

interface ITreeMenu {
  id?: string,
  title: string | JSX.Element,
  path: string,
}

export enum TreeMenuList {

  Root = "",

  Payment = "payment",
  MakePayment = "make-payment",
  ListPayment = "list-payment",
  EditPayment = "edit-payment",

  TakingRecord = "taking-record",
  UpdateTakingRecord = "update-taking-record",
  TakingRecordInformation = "taking-record-info",

  CustomerManagement = "customer-management",
  EditCustomer = "edit-customer",
  CreateCustomer = "create-customer",
  UserManagement = "user-management",
}

const paymentMenu: Array<ITreeMenu> = [
  {
    title:
      <>
        <AiOutlineDollar />
        <Text>Payment Menu</Text>
      </>,
    path: TreeMenuList.Payment,
  },
  {
    title:
      <>
        <AiOutlineShoppingCart />
        <Text>Make Payment</Text>
      </>,
    path: `${TreeMenuList.Payment}/${TreeMenuList.MakePayment}`,
    id: `${AdminRoutes.AdminRoot}${AdminRoutes.MakePaymentPage}`
  },
  {
    title:
      <>
        <AiFillDatabase />
        <Text>List Payment</Text>
      </>,
    path: `${TreeMenuList.Payment}/${TreeMenuList.ListPayment}`,
    id: `${AdminRoutes.AdminRoot}${AdminRoutes.ListPaymentPage}`
  },
  // {
  //   title:
  //     <>
  //       <AiFillEdit />
  //       <Text>Edit Payment</Text>
  //     </>,
  //   path: `${TreeMenuList.Payment}/${TreeMenuList.EditPayment}`,
  // },
]

const takingRecordMenu: Array<ITreeMenu> = [
  {
    title:
      <>
        <AiFillDropboxSquare />
        <Text>Taking Menu</Text>
      </>,
    path: TreeMenuList.TakingRecord,
  },
  {

    title:
      <>
        <AiFillApi />
        <Text>Update Taking Record</Text>
      </>,
    path: `${TreeMenuList.TakingRecord}/${TreeMenuList.UpdateTakingRecord}`,
    id: `${AdminRoutes.AdminRoot}${AdminRoutes.UpdateTakingRecord}`
  },
  // {
  //   title:
  //     <>
  //       <AiFillInfoCircle />
  //       <Text>Taking Record Info</Text>
  //     </>,
  //   path: `${TreeMenuList.TakingRecord}/${TreeMenuList.TakingRecordInformation}`,
  // },
]

const customerManagement: Array<ITreeMenu> = [
  {
    title:
      <>
        <AiFillCreditCard />
        <Text>User Management</Text>
      </>,
    path: `${TreeMenuList.CustomerManagement}/${TreeMenuList.UserManagement}`,
    id: `${AdminRoutes.AdminRoot}${AdminRoutes.UserManagementPage}`
  },
  {
    title:
      <>
        <AiOutlineUser />
        <Text>Customer Management</Text>
      </>,
    path: TreeMenuList.CustomerManagement,
  },
  // {
  //   title:
  //     <>
  //       <AiOutlineUserAdd />
  //       <Text>Create Customer</Text>
  //     </>,
  //   path: `${TreeMenuList.CustomerManagement}/${TreeMenuList.CreateCustomer}`,
  // },
  // {
  //   title:
  //     <>
  //       <AiOutlineUserSwitch />
  //       <Text>Edit Customer</Text>
  //     </>,
  //   path: `${TreeMenuList.CustomerManagement}/${TreeMenuList.EditCustomer}`,
  // },
]

export const defaultTreeMenu: Array<ITreeMenu> = [
  ...paymentMenu,
  ...takingRecordMenu,
  ...customerManagement
]

export default function TreeMenuComponent() {

  const menuPath = useAdminPageStore(state => state.menuPath)
  const setMenuPath = useAdminPageStore(state => state.setMenuPath)

  const treeMenuVisible = useAdminPageStore(state => state.treeMenuButtonVisible)
  const setTreeMenuVisibility = useAdminPageStore(state => state.setTreeMenuButtonVisible)

  const handleOnNavigateButtonClicked = (path: string) => {
    setMenuPath(path)
  }

  const renderTreeMenu = (menus: Array<ITreeMenu>) => {

    if (menuPath === TreeMenuList.Root) {
      const rootMenu = menus.filter(pr => !pr.path.includes('/'))

      return (
        <>
          {rootMenu.map((item, index) => (
            <Button
              key={index}
              onClick={() => handleOnNavigateButtonClicked(item.path)}>
              {item.title}
            </Button>
          ))}
        </>
      )
    }

    else {
      const level = menuPath.split('/').length
      const subMenus = menus.filter(pr =>
        pr.path.startsWith(menuPath)
        && pr.path !== menuPath
        && pr.path.split('/').length === level + 1
      )

      return (
        <>
          {subMenus.map((item, index) => (
            <Button
              key={index}
              onClick={() => handleOnNavigateButtonClicked(item.path)}>
              {item.title}
            </Button>
          ))}
        </>
      )
    }
  }

  const handleOnBackButtonClicked = (path: string) => {
    if (!path.includes('/')) setMenuPath(TreeMenuList.Root)
    else {
      const parentPath = path.split('/').slice(0, -1)
      setMenuPath(parentPath.join('/'))
    }
  }

  useEffect(() => {
    setTreeMenuVisibility(!(menuPath === TreeMenuList.Root))
  }, [menuPath])

  return (
    <Stack
      className='tree-menu'
    >
      {
        treeMenuVisible
        && (
          <IconButton
            onClick={() => handleOnBackButtonClicked(menuPath)}>
            <AiFillCaretLeft />
            Back
          </IconButton>
        )
      }

      {renderTreeMenu(defaultTreeMenu)}
    </Stack>
  )
}
