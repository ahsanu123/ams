import type { MenuTree } from "@/model"
import { useAdminPageStore } from "@/state"
import React from "react"
import './MenuTree.css'

export default function MenuTreeComponent() {

  const adminMenus = useAdminPageStore(state => state.adminMenus)
  const activeMenu = useAdminPageStore(state => state.activeAdminMenu)
  const setActiveMenu = useAdminPageStore(state => state.setActiveAdminMenu)

  const handleOnSelectedArrayMenuChanged = (key: string) => {
    const menuTree = adminMenus.find(pr => pr.key == key)
    if (menuTree == undefined) return
    setActiveMenu(menuTree)
  }

  const handleOnSelectedMenuChanged = (key: string) => {
    const menuTree = activeMenu?.children?.find(pr => pr.key == key)
    if (menuTree == undefined) return
    setActiveMenu(menuTree)
  }

  const displayMenu = (menus: MenuTree[] | MenuTree) => {
    if (Array.isArray(menus))

      return (
        <>
          {menus.map((menu, menuKey) => (
            <ul
              className="list-menu"
              key={menuKey}>
              {
                menu.isRoot == undefined
                  ? <button>Back</button>
                  : (
                    <button
                      onClick={() => handleOnSelectedArrayMenuChanged(menu.key)}
                    >
                      {menu.key}
                    </button>
                  )
              }
            </ul>
          ))}
        </>
      )
    else return (
      <>
        <ul
          className="list-menu"
        >
          <button
            className="cmd-button"
            onClick={() => setActiveMenu(undefined)}
          >
            Root Menu
          </button>
        </ul >
        {
          menus.children ?
            menus.children.map((child, index) =>
            (
              <React.Fragment
                key={index}>
                <ul
                  className="list-menu"
                >
                  <button
                    onClick={() => handleOnSelectedMenuChanged(child.key)}>
                    {child.key}
                  </button>

                  {child.children != undefined && displayMenu(child.children)}
                </ul>
              </React.Fragment>
            )
            )
            : <></>
        }
      </>
    )
  }


  return (
    <div
      className="menu-tree"
    >
      {
        activeMenu
          ? displayMenu(activeMenu)
          : displayMenu(adminMenus)
      }
    </div>
  )
}
