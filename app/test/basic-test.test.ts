import { useMainAdminPageState } from "@/state";
import { test, expect, describe, beforeAll } from "vitest";

describe("ProductListComponentTest", () => {
  beforeAll(() => {
    useMainAdminPageState.getState().reset()
  });

  test("mathMorks", () => {
    console.log("finished")
    process.stdout.write("hello")
    expect(1 + 1).toBe(2)
  });

  test("async test", async () => {
    const v = await Promise.resolve(42)
    expect(v).toBe(42)
  });
})
