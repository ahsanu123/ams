import type { DialogModel } from "@/model"
import { create } from "zustand"
import { immer } from "zustand/middleware/immer"


interface EditPageStore {
  dialogData?: DialogModel,
  setDialogData: (data: DialogModel) => void,

  isDialogVisible: boolean,
  setDialogVisibility: (visible: boolean) => void
}

export const useEditPageStore = create<EditPageStore>()(
  immer((set) => ({

    setDialogData: (data) => {
      set((state) => {
        state.dialogData = data
      })
    },

    isDialogVisible: false,
    setDialogVisibility: (visible) => {

      set((state) => {
        state.isDialogVisible = visible
      })

    },

  }))
)
