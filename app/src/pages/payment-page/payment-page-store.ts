import { Customer } from '@/api/v1/models'
import { addMonths, subMonths } from 'date-fns'
import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface PaymentPageState {
  isShowCalendar: boolean
  selectedMonth: Date,
  selectedCustomer?: Customer
}

interface PaymentPageStore extends PaymentPageState {
  setIsShowCalendar: (isShowCalendar: boolean) => void,
  setSelectedMonth: (date: Date) => void,
  increaseMonth: () => void,
  decreaseMonth: () => void,
  setSelectedCustomer: (customer?: Customer) => void,
}

const paymentPageStoreDefaultValue: PaymentPageState = {
  isShowCalendar: false,
  selectedMonth: new Date()
}


export const usePaymentPageStore = create<PaymentPageStore>()(
  immer((set, get) => ({
    ...paymentPageStoreDefaultValue,

    setIsShowCalendar: (isShowCalendar: boolean) => {
      set((state) => {
        state.isShowCalendar = isShowCalendar;
      })
    },

    setSelectedMonth: (date: Date) => {
      set((state) => {
        state.selectedMonth = date
      })
    },

    increaseMonth: () => {
      set((state) => {
        state.selectedMonth = addMonths(state.selectedMonth, 1)
      })
    },

    decreaseMonth: () => {
      set((state) => {
        state.selectedMonth = subMonths(state.selectedMonth, 1)
      })
    },

    setSelectedCustomer: (customer?: Customer) => {
      set((state) => {
        state.selectedCustomer = customer
      })
    }

  }))
)
