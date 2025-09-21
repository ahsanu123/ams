import type { User } from "@/model"
import { IS_INSIDE_TAURI } from "@/constants"
import type { UserInvokeCommand } from "./user-invoke-interface"
import { mockUserCommand } from "@/mock"

const realCommand: UserInvokeCommand = {
  getUserById: function (id: number): Promise<User> {
    throw new Error("Function not implemented.")
  },
  getUsers: function (): Promise<User[]> {
    throw new Error("Function not implemented.")
  },
}

export const getUserCommand = (): UserInvokeCommand => {
  if (IS_INSIDE_TAURI) return realCommand
  else return mockUserCommand
}
