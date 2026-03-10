import { dregPriceCommand } from "@/commands";
import { test, expect, describe, beforeAll } from "vitest";

describe("dreg-api-test", () => {
  beforeAll(() => {
    console.log("beforeAll")
  });

  test("get_all_dreg_price", async () => {
    const allPrice = await dregPriceCommand.getAllDregPrice()
    console.log(allPrice)
  });

  test("get_latest_dreg_price", async () => {
    const latestPrice = await dregPriceCommand.getLatestDregPrice()
    console.log(latestPrice)
  });

  test("update_dreg_price", async () => {
    const updatedPrice = await dregPriceCommand.updateDregPrice(20000);
    console.log(updatedPrice)
  });
})
