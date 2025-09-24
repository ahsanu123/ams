import { useMainAdminPageState } from "@/state";
import { test, expect, describe, beforeAll } from "vitest";

describe("mainAdminPageStateTest", () => {
  beforeAll(() => {
    useMainAdminPageState.getState().reset()
  })

  test("listAllMenu", () => {
    const menus = useMainAdminPageState.getState().allMenu
    expect(menus).not.empty

    console.log(menus)
  })

})
