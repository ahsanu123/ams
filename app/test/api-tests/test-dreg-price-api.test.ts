import { dregPriceApi } from "@/commands";
import { test, expect, describe, beforeAll } from "vitest";

describe("dreg-api-test", () => {
  beforeAll(() => {
    console.log("beforeAll")
  });

  test("get_all_dreg_price", async () => {
    const allPrice = await dregPriceApi.getAllDregPrice()
    console.log(allPrice)
  });

  test("get_latest_dreg_price", async () => {
    const latestPrice = await dregPriceApi.getLatestDregPrice()
    console.log(latestPrice)
  });

  test("update_dreg_price", async () => {
    const updatedPrice = await dregPriceApi.updateDregPrice(20000);
    console.log(updatedPrice)
  });
})
