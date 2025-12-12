import { create } from "zustand"
import { immer } from "zustand/middleware/immer"
import { defaultTreeMenu } from "./components"

interface AdminPageStore {
  treeMenuButtonVisible: boolean,
  setTreeMenuButtonVisible: (visible: boolean) => void,

  menuPathId?: string,
  menuPath: string,
  setMenuPath: (path: string) => void
}

export const useAdminPageStore = create<AdminPageStore>()(
  immer((set) => ({
    menuPathId: undefined,

    treeMenuButtonVisible: false,
    setTreeMenuButtonVisible: (visible) => {
      set((state) => {
        state.treeMenuButtonVisible = visible
      })
    },

    menuPath: "",
    setMenuPath: (path) => {
      set((state) => {
        state.menuPathId = defaultTreeMenu.find(pr => pr.path === path)?.id
        state.menuPath = path
      })
    }

  }))
)
