import { dregPriceCommand } from "@/commands";
import { fetchWithDateTimeCompitable } from "@/commands/fetch-wrapper";
import { test, expect, describe, beforeAll } from "vitest";

describe("fetch_wrapper", () => {

  test("fetch_with_date_compatibility", async () => {
    fetchWithDateTimeCompitable("some", {
      body: JSON.stringify({
        dat: new Date()
      })
    })
  });
})
