import type { UserModel } from "@/api-models";
import { userManagementCommand } from "@/commands";
import { test, expect, describe, beforeAll } from "vitest";

describe("user_management_api_test", () => {
  test("get_all_active_user", async () => {
    const result = await userManagementCommand.getAllActiveUser()

    console.log(result)
  });

  test("get_all_user", async () => {
    const result = await userManagementCommand.getAllUser()

    console.log(result)
  });

  test("insert_new_user", async () => {
    const newUser: UserModel = {
      id: undefined,
      username: "Brisbane",
      isActive: true,
      isAdmin: false,
      money: 0n,
      createdDate: new Date(),
      updatedDate: new Date()
    }
    const result = await userManagementCommand.insertNewUser(newUser)

    console.log(result)
  });

  test("upsert_user", async () => {
    const newUser: UserModel = {
      id: undefined,
      username: "adelaide",
      isActive: true,
      isAdmin: false,
      money: 0n,
      createdDate: new Date(),
      updatedDate: new Date()
    }
    const result = await userManagementCommand.UpsertUser(newUser)

    console.log(result)
  });
})
