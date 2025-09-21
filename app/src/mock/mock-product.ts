import type { ProductInvokeCommand } from "@/commands";
import type { Product, User } from "@/model";
import { isSameMonth } from "date-fns";

const mockProductRecord: Array<Product> = [
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
    userId: 2,
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

export const mockProductCommand: ProductInvokeCommand = {
  getProductRecord: (): Promise<Array<Product>> => {
    return Promise.resolve(mockProductRecord);
  },

  getProductPrice: function (): Promise<number> {
    return Promise.resolve(11000);
  },

  getAllProductOfThisMonth: function (date: Date): Promise<Array<Product>> {
    const productOfThisMonth = mockProductRecord.filter((product) => isSameMonth(product.takenDate, date))
    return Promise.resolve(productOfThisMonth)
  }
}
