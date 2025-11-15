import type { MakePaymentPageModel, UserModel } from '@/api-models'
import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface IUserManagementPageState {
  newUserName: string,
  setNewUserName: (name: string) => void
}

export const useUserManagementPageState = create<IUserManagementPageState>()(
  immer((set, get) => ({

    newUserName: '',
    setNewUserName: (name: string) => {
      set((state) => {
        state.newUserName = name
      })
    }

  }))
)
