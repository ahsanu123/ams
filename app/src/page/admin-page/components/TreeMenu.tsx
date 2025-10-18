import { useEffect } from 'react';
import './TreeMenu.css'
import { useAdminPageStore } from '../admin-page-state';
import { AdminRoutes, AppRoutes } from '@/routes';

interface ITreeMenu {
  id?: string,
  title: string,
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
  CustomerMoney = "customer-money",
}

const paymentMenu: Array<ITreeMenu> = [
  {
    title: 'Payment Menu',
    path: TreeMenuList.Payment,
  },
  {
    title: 'Make Payment',
    path: `${TreeMenuList.Payment}/${TreeMenuList.MakePayment}`,
    id: `${AdminRoutes.AdminRoot}${AdminRoutes.MakePaymentPage}`
  },
  {
    title: 'List Payment',
    path: `${TreeMenuList.Payment}/${TreeMenuList.ListPayment}`,
  },
  {
    title: 'Edit Payment',
    path: `${TreeMenuList.Payment}/${TreeMenuList.EditPayment}`,
  },
]

const takingRecordMenu: Array<ITreeMenu> = [
  {
    title: 'Taking Menu',
    path: TreeMenuList.TakingRecord,
  },
  {
    title: 'Update Taking Record',
    path: `${TreeMenuList.TakingRecord}/${TreeMenuList.UpdateTakingRecord}`,
  },
  {
    title: 'Taking Record Info',
    path: `${TreeMenuList.TakingRecord}/${TreeMenuList.TakingRecordInformation}`,
  },
]

const customerManagement: Array<ITreeMenu> = [
  {
    title: 'Customer Management',
    path: TreeMenuList.CustomerManagement,
  },
  {
    title: 'Create Customer',
    path: `${TreeMenuList.CustomerManagement}/${TreeMenuList.CreateCustomer}`,
  },
  {
    title: 'Edit Customer',
    path: `${TreeMenuList.CustomerManagement}/${TreeMenuList.EditCustomer}`,
  },
  {
    title: 'Customer Money',
    path: `${TreeMenuList.CustomerManagement}/${TreeMenuList.CustomerMoney}`,
  },
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
            <button
              key={index}
              onClick={() => handleOnNavigateButtonClicked(item.path)}>
              {item.title}
            </button>
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
            <button
              key={index}
              onClick={() => handleOnNavigateButtonClicked(item.path)}>
              {item.title}
            </button>
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

  const handleOnBreadCrumbsClick = (item: string) => {
    const menu = defaultTreeMenu.find(pr => pr.path.endsWith(item))
    if (menu !== undefined) setMenuPath(menu.path)
  }

  // TODO:
  const breadCrumbs = () => (
    <>
      {
        menuPath !== TreeMenuList.Root
        && menuPath.split('/').map((item) => (
          <button
            className='bread-crumbs'
            onClick={() => handleOnBreadCrumbsClick(item)}
          >
            {item}
          </button>
        ))
      }
    </>
  )
  return (
    <div className='tree-menu'>
      {
        treeMenuVisible
        && (
          <button
            onClick={() => handleOnBackButtonClicked(menuPath)}
          >
            Back
          </button>
        )
      }

      {renderTreeMenu(defaultTreeMenu)}

    </div>
  )
}
