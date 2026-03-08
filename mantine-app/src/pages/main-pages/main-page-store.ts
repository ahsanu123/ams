import { Customer } from '@/bindings/Customer'
import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface MainPageState {
  selectedCustomer?: Customer,
}

interface MainPageStore extends MainPageState {
  setSelectedCustomer: (customer?: Customer) => void,
}

const mainPageStoreDefaultValue: MainPageState = {

}


export const useSidebarStore = create<MainPageStore>()(
  immer((set, get) => ({
    ...mainPageStoreDefaultValue,

    setSelectedCustomer: (customer?: Customer) => {
      set((state) => {
        state.selectedCustomer = customer;
      })
    }

  }))
)
