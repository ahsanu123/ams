import { customerMoney } from "@/commands";
import { test, expect, describe, beforeAll } from "vitest";

describe("test_customer_money_api", () => {
  test("add_money", async () => {
    const response = await customerMoney.addMoney(1, 10000);
    console.log(response)
  });

  test("get_all_user_money_history", async () => {
    const response = await customerMoney.getAllUserMoneyHistory(1)
    console.log(response)
  });
})
