import type { MakePaymentPageModel, PaymentHistoryModel, TakingRecordModel, UserModel } from '@/api-models'
import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface IListPaymentPage {
  paymentRecords: PaymentHistoryModel[],
  setPaymentRecords: (records: PaymentHistoryModel[]) => void,

  pageModel?: MakePaymentPageModel,
  setPageModel: (pageModel: MakePaymentPageModel) => void,

  showDetailTaking: boolean,
  setShowDetailTaking: (shown: boolean) => void,

  selectedDate?: Date
  setSelectedDate: (date?: Date) => void,

  selectedCustomer?: UserModel
  setSelectedCustomer: (customer?: UserModel) => void,

  listCustomer: Array<UserModel>,
  setListCustomer: (customers: Array<UserModel>) => void,

  dateSelectorEnabled: boolean,
}

export const useListPaymentPageState = create<IListPaymentPage>()(
  immer((set, get) => ({

    paymentRecords: [],
    setPaymentRecords: (records: PaymentHistoryModel[]) => {
      set((state) => {
        state.paymentRecords = records
      })
    },

    pageModel: undefined,
    setPageModel: (pageModel: MakePaymentPageModel) => {
      set((state) => {
        state.pageModel = pageModel
      })
    },

    showDetailTaking: false,
    setShowDetailTaking: (shown: boolean) => {
      set((state) => {
        state.showDetailTaking = shown
      })
    },

    selectedDate: undefined,
    setSelectedDate: (date) => {
      set((state) => {
        state.selectedDate = date
      })
    },

    selectedCustomer: undefined,
    setSelectedCustomer: (customer) => {
      set((state) => {
        state.selectedCustomer = customer
      })
    },

    listCustomer: [],
    setListCustomer: (customers: Array<UserModel>) => {
      set((state) => {
        state.listCustomer = customers
      })
    },

    dateSelectorEnabled: false,
  }))
)
