import { test, expect, describe, beforeAll } from "vitest";

describe("dreg-api-test", () => {
  beforeAll(() => {
    console.log("beforeAll")
  });

  test("async test", async () => {
    const v = await Promise.resolve(42)
    expect(v).toBe(42)
  });
})
