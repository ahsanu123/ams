import { defaultFlatMenuTree } from "@/constants"
import type { FlatMenuTree } from "@/model"
import { create } from "zustand"
import { immer } from "zustand/middleware/immer"
import type { IBaseState } from "./base-state-interface"

interface MainAdminPageState extends IBaseState {

  menus: FlatMenuTree[],

  activeMenu: FlatMenuTree,
  setActiveMenu: (menu: FlatMenuTree) => void,

  getSubMenu: (path: string) => FlatMenuTree[]

}

export const useMainAdminPageState = create<MainAdminPageState>()(
  immer((set, get) => ({

    reset: () => {
      set((state) => {
        state.activeMenu = defaultFlatMenuTree[0]
      })
    },

    menus: defaultFlatMenuTree,
    activeMenu: defaultFlatMenuTree[0],

    setActiveMenu: (menu) => {
      set((state) => {
        state.activeMenu = menu
      })
    },

    getSubMenu: (searchPath) => get().menus.filter(pr => pr.path.startsWith(searchPath))

  }))
)
