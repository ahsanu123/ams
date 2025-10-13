import { takingRecord } from "@/commands";
import { test, expect, describe, beforeAll } from "vitest";

const currentDate = new Date()

describe("taking_record_api_test", () => {
  test("add_new_taking_record", async () => {
    const result = await takingRecord.addNewTakingRecord(1, 2);

    console.log(result)
  });

  test("get_taking_Record_by_month", async () => {
    const result = await takingRecord.getTakingRecordByMonth(currentDate)

    console.log(result)
  });

  test("get_taking_record_by_user_id", async () => {
    const result = await takingRecord.getTakingRecordByUserId(1)

    console.log(result)
  });

  test("get_taking_record_by_user_id_and_month", async () => {
    const result = await takingRecord.getTakingRecordByUserIdAndMonth(1, currentDate)

    console.log(result)
  });

  test("upsert_taking_record", async () => {
    // TODO:
    // const result = await takingRecordApi.upsertTakingRecord()
    // console.log(result)
  });
})
