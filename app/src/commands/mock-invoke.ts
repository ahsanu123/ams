import type { ProductRecord, User } from "@/model";
import type { InvokeCommand } from "./invoke-interface";

const mockUser: User[] = [
  {
    id: 1,
    username: "Paijo",
    isActive: true,
    money: 1000000,
    bill: 0,
    isAdmin: false
  },
  {
    id: 2,
    username: "Tukiman",
    isActive: true,
    money: 1000000,
    bill: 0,
    isAdmin: false
  },
  {
    id: 3,
    username: "Painem",
    isActive: true,
    money: 1000000,
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

const mockProductRecord: ProductRecord = [
  {
    id: 1,
    userId: 1,
    paid: true,
    productionDate: new Date(2025, 2, 2),
    takenDate: new Date(2025, 2, 2),
    price: 11000,
    amount: 1,
    description: "taking 1"
  },
  {
    id: 1,
    userId: 1,
    paid: false,
    productionDate: new Date(2025, 2, 10),
    takenDate: new Date(2025, 2, 10),
    price: 11000,
    amount: 2,
    description: "taking 1"
  },
  {
    id: 1,
    userId: 1,
    paid: false,
    productionDate: new Date(2025, 2, 11),
    takenDate: new Date(2025, 2, 11),
    price: 11000,
    amount: 3,
    description: "taking 1"
  },
  {
    id: 1,
    userId: 1,
    paid: false,
    productionDate: new Date(2025, 2, 12),
    takenDate: new Date(2025, 2, 12),
    price: 11000,
    amount: 4,
    description: "taking 1"
  },
  {
    id: 1,
    userId: 1,
    paid: false,
    productionDate: new Date(2025, 2, 13),
    takenDate: new Date(2025, 2, 13),
    price: 11000,
    amount: 6,
    description: "taking 1"
  },
  {
    id: 1,
    userId: 1,
    paid: false,
    productionDate: new Date(2025, 3, 25),
    takenDate: new Date(2025, 3, 25),
    price: 11000,
    amount: 2,
    description: "taking 1"
  },
  {
    id: 1,
    userId: 1,
    paid: false,
    productionDate: new Date(2025, 3, 1),
    takenDate: new Date(2025, 3, 1),
    price: 11000,
    amount: 2,
    description: "taking 1"
  }
]

export const mockCommand: InvokeCommand = {
  getProductRecord: (): Promise<ProductRecord> => {
    return Promise.resolve(mockProductRecord);
  },

  getUserById: function (id: number): Promise<User> {
    const user = mockUser.find((user) => user.id === id);

    if (user) return Promise.resolve(user);
    else return Promise.reject("User not found");
  },

  getUsers: function (): Promise<User[]> {
    const userNoAdmin = mockUser.filter((user) => !user.isAdmin)
    return Promise.resolve(userNoAdmin)
  }
}
