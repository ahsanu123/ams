import { title } from 'node:process'
import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

export enum SideBarComponentType {
  None,
  PaymentPage,
  DashboardPage,
  AdminLogin,
  Statistics,
  MainPageCustomerPicker,
}

interface SideBarState {
  displayedComponent: SideBarComponentType,
  title: string,
}

interface SideBarStore extends SideBarState {
  setDisplayedComponent: (componentType: SideBarComponentType) => void,
  setTitle: (title: string) => void,
}

const layoutState: SideBarState = {
  displayedComponent: SideBarComponentType.MainPageCustomerPicker,
  title: ""
}


export const useSidebarStore = create<SideBarStore>()(
  immer((set, get) => ({
    ...layoutState,

    setDisplayedComponent: (componentType: SideBarComponentType) => {
      set((state) => {
        state.displayedComponent = componentType;
      })
    },

    setTitle: (title: string) => {
      set((state) => {
        state.title = title;
      })
    }

  }))
)
