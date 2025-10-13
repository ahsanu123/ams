import { defaultFlatMenuTree } from "@/constants"
import type { FlatMenuTree } from "@/model"
import { create } from "zustand"
import { immer } from "zustand/middleware/immer"
import type { IBaseState } from "./base-state-interface"

interface MainAdminPageState extends IBaseState {

  allMenu: FlatMenuTree[],

  selectedMenu?: FlatMenuTree,
  setSelectedMenu: (menu: FlatMenuTree) => void,

  // activeMenu: FlatMenuTree[],
  setActiveMenu: (path: string) => void,

  getSubMenu: (path: string) => FlatMenuTree[]
  getRootMenu: () => FlatMenuTree[],

  setActiveMenuBackToRoot: () => void
}

export const useMainAdminPageState = create<MainAdminPageState>()(
  immer((set, get) => ({

    reset: () => {
      set((state) => {
        // state.activeMenu = state.getRootMenu()
      })
    },

    allMenu: defaultFlatMenuTree,
    // activeMenu: defaultFlatMenuTree.at(0),

    setActiveMenu: (path) => {
      set((state) => {
        // state.activeMenu = state.allMenu.filter(pr => pr.path.startsWith(path))
      })
    },

    selectedMenu: undefined,
    setSelectedMenu: (menu) => {
      set((state) => {
        state.selectedMenu = menu
      })
    },

    getSubMenu: (searchPath) => get().allMenu.filter(pr => pr.path.startsWith(searchPath)),
    getRootMenu: () => get().allMenu.filter(pr => pr.isRoot),
    setActiveMenuBackToRoot: () => {
      set((state) => {
        // state.activeMenu = defaultFlatMenuTree.filter(pr => pr.isRoot)
      })
    }

  }))
)
