// TODO: add return type
interface ITakingRecordApi {
  addNewTakingRecord: (userId: number, amount: number) => void
  getTakingRecordByUserId: (userId: number) => void
  upsertTakingRecord: (/*request_model::UpdateTakingRecord*/) => void
  getTakingRecordByMonth: (date: Date) => void
  getTakingRecordByUserIdAndMonth: (userId: number, date: Date) => void
}


export const takingRecordApi: ITakingRecordApi = {
  addNewTakingRecord: function (userId: number, amount: number): void {
    throw new Error("Function not implemented.")
  },
  getTakingRecordByUserId: function (userId: number): void {
    throw new Error("Function not implemented.")
  },
  upsertTakingRecord: function (): void {
    throw new Error("Function not implemented.")
  },
  getTakingRecordByMonth: function (date: Date): void {
    throw new Error("Function not implemented.")
  },
  getTakingRecordByUserIdAndMonth: function (userId: number, date: Date): void {
    throw new Error("Function not implemented.")
  }
}
