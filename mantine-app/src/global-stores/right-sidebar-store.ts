import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface SideBarState {
  title: string,
}

interface SideBarStore extends SideBarState {
  setTitle: (title: string) => void,
}

const layoutState: SideBarState = {
  title: 'right sidebar'
}


export const useSidebarStore = create<SideBarStore>()(
  immer((set, get) => ({
    ...layoutState,

    setTitle: (title: string) => {
      set((state) => {
        state.title = title;
      })
    }

  }))
)
