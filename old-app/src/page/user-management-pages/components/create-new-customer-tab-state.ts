import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface ICreateNewCustomerTabState {
  newUserName: string,
  setNewUserName: (name: string) => void
}

export const useCreateNewCustomerTabState = create<ICreateNewCustomerTabState>()(
  immer((set, get) => ({

    newUserName: '',
    setNewUserName: (name: string) => {
      set((state) => {
        state.newUserName = name
      })
    }

  }))
)
