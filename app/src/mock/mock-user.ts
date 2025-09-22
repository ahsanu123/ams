import type { UserInvokeCommand } from "@/commands/user-invoke-interface";
import type { User } from "@/model";

const mockUser: User[] = [

  {
    id: 1,
    username: "JOHN",
    isActive: true,
    money: 4000000,
    bill: 0,
    isAdmin: false
  },
  {
    id: 1,
    username: "BARN",
    isActive: true,
    money: 4000000,
    bill: 0,
    isAdmin: false
  },
  {
    id: 1,
    username: "BRISBANE",
    isActive: true,
    money: 4000000,
    bill: 0,
    isAdmin: false
  },
  {
    id: 1,
    username: "ADELAIDE",
    isActive: true,
    money: 4000000,
    bill: 0,
    isAdmin: false
  },
  {
    id: 2,
    username: "MELBOURNE",
    isActive: true,
    money: 2500000,
    bill: 0,
    isAdmin: false
  },
  {
    id: 3,
    username: "NEW YORK",
    isActive: true,
    money: 6000000,
    bill: 0,
    isAdmin: false
  },
  {
    id: 4,
    username: "Admin",
    isActive: true,
    money: 0,
    bill: 0,
    isAdmin: true
  }
]

export const mockUserCommand: UserInvokeCommand = {
  getUserById: function (id: number): Promise<User> {
    const user = mockUser.find((user) => user.id === id);

    if (user) return Promise.resolve(user);
    else return Promise.reject("User not found");
  },

  getUsers: function (): Promise<User[]> {
    const userNoAdmin = mockUser.filter((user) => !user.isAdmin);
    return Promise.resolve(userNoAdmin);
  },
}
