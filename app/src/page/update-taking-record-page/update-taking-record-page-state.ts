import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface IUpdateTakingRecordState {
}

export const useMakePaymentPageState = create<IUpdateTakingRecordState>()(
  immer((set, get) => ({


  }))
)
