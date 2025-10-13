import { customerMoneyApi } from "@/commands";
import { test, expect, describe, beforeAll } from "vitest";

describe("test_customer_money_api", () => {
  test("add_money", async () => {
    const response = await customerMoneyApi.addMoney(1, 10000);
    console.log(response)
  });

  test("get_all_user_money_history", async () => {
    const response = await customerMoneyApi.getAllUserMoneyHistory(1)
    console.log(response)
  });
})
