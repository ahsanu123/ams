import type { TakingRecordModel, UserModel } from '@/api-models'
import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface IUpdateTakingRecordState {

  isAddingNewRecord: boolean,
  setIsAddingAddNewRecord: (status: boolean) => void,

  customers: UserModel[],
  setCustomers: (customers: UserModel[]) => void,

  selectedDate?: Date,
  setSelectedDate: (date?: Date) => void,

  takingRecords: TakingRecordModel[],
  setTakingRecord: (records: TakingRecordModel[]) => void,

  selectedTakingRecord?: TakingRecordModel,
  setSelectedTakingRecord: (record?: TakingRecordModel) => void,

  selectedCustomer?: UserModel,
  setSelectedCustomer: (customer?: UserModel) => void,
}

export const useUpdateTakingPageState = create<IUpdateTakingRecordState>()(
  immer((set, get) => ({
    isAddingNewRecord: false,
    setIsAddingAddNewRecord: (status: boolean) => {
      set((state) => {
        state.isAddingNewRecord = status
      })
    },

    customers: [],
    setCustomers: (customers: UserModel[]) => {
      set((state) => {
        state.customers = customers
      })
    },

    selectedDate: undefined,
    setSelectedDate: (date?: Date) => {
      set((state) => {
        state.selectedDate = date
      })
    },

    takingRecords: [],
    setTakingRecord: (records: TakingRecordModel[]) => {
      set((state) => {
        state.takingRecords = records
      })
    },

    selectedTakingRecord: undefined,
    setSelectedTakingRecord: (record?: TakingRecordModel) => {
      set((state) => {
        state.selectedTakingRecord = record
      })
    },

    selectedCustomer: undefined,
    setSelectedCustomer: (customer?: UserModel) => {
      set(state => {
        state.selectedCustomer = customer
      })
    }

  }))
)
