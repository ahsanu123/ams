import { addYears } from "date-fns"
import { create } from "zustand"
import { immer } from "zustand/middleware/immer"

interface AdminPageStore {
  selectedYear: Date,
  setSelectedYear: (date: Date) => void,
  increaseYear: () => void,
  decreaseYear: () => void,
}

export const useAdminPageStore = create<AdminPageStore>()(
  immer((set) => ({

    selectedYear: new Date(),

    setSelectedYear: (date) => {
      set((state) => {
        state.selectedYear = date
      })
    },

    increaseYear: () => {
      set((state) => {
        state.selectedYear = addYears(state.selectedYear, 1)
      })
    },

    decreaseYear: () => {
      set((state) => {
        state.selectedYear = addYears(state.selectedYear, -1)
      })
    }

  }))
)
