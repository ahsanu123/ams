import { useEffect } from 'react';
import './TreeMenu.css'
import { useAdminPageStore } from '../admin-page-state';
import { AdminRoutes, AppRoutes } from '@/routes';

interface ITreeMenu {
  id?: string,
  title: string,
  path: string,
}

export enum ETreeMenuList {

  Root = "",

  Payment = "payment",
  SubPayment1 = "sub-payment1",
  SubPayment2 = "sub-payment2",
  SubSubPayment2 = "sub-sub-payment2",
  SubSubSubPayment2 = "sub-sub-sub-payment2",

  Edit = "edit",
  SubEdit1 = "sub-edit1",
  SubEdit2 = "sub-edit2",
}

const paymentMenu: Array<ITreeMenu> = [
  {
    title: 'Payment Menu',
    path: ETreeMenuList.Payment,
  },
  {
    title: 'Sub Payment Menu 1',
    path: `${ETreeMenuList.Payment}/${ETreeMenuList.SubPayment1}`,
  },
  {
    title: 'Sub Payment Menu 2',
    path: `${ETreeMenuList.Payment}/${ETreeMenuList.SubPayment2}`,
  },
  {
    title: 'Sub sub Payment Menu 2',
    path: `${ETreeMenuList.Payment}/${ETreeMenuList.SubPayment2}/${ETreeMenuList.SubSubPayment2}`,
  },
  {
    id: `${AdminRoutes.AdminRoot}${AdminRoutes.ReportPage}`,
    title: 'weird menu title',
    path: `${ETreeMenuList.Payment}/${ETreeMenuList.SubPayment2}/${ETreeMenuList.SubSubPayment2}/${ETreeMenuList.SubSubSubPayment2}`,
  },
]

const editMenu: Array<ITreeMenu> = [
  {
    title: 'Edit Menu',
    path: ETreeMenuList.Edit,
  },
  {
    title: 'Sub edit Menu 1',
    path: `${ETreeMenuList.Edit}/${ETreeMenuList.SubEdit1}`,
  },
  {
    title: 'Sub edit Menu 2',
    path: `${ETreeMenuList.Edit}/${ETreeMenuList.SubEdit2}`,
  },
]

export const defaultTreeMenu: Array<ITreeMenu> = [
  ...paymentMenu,
  ...editMenu
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

    if (menuPath === ETreeMenuList.Root) {
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
    if (!path.includes('/')) setMenuPath(ETreeMenuList.Root)
    else {
      const parentPath = path.split('/').slice(0, -1)
      setMenuPath(parentPath.join('/'))
    }
  }

  useEffect(() => {
    setTreeMenuVisibility(!(menuPath === ETreeMenuList.Root))
  }, [menuPath])

  const handleOnBreadCrumbsClick = (item: string) => {
    const menu = defaultTreeMenu.find(pr => pr.path.endsWith(item))
    if (menu !== undefined) setMenuPath(menu.path)
  }

  // TODO:
  const breadCrumbs = () => (
    <>
      {
        menuPath !== ETreeMenuList.Root
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
