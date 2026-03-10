import { paymentHistoryCommand } from "@/commands";
import { test, expect, describe, beforeAll } from "vitest";

const currentDate = new Date()

describe("test_payment_history_api", () => {
  test("get_month_summary", async () => {
    const result = await paymentHistoryCommand.getMonthSummary(currentDate)

    console.log(result)
  });

  test("get_payment_record", async () => {
    const result = await paymentHistoryCommand.getPaymentRecord(1)

    console.log(result)
  });

  test("get_payment_record_by_user_id_and_month", async () => {
    const result = await paymentHistoryCommand.getPaymentRecordByUserIdAndMonth(1, currentDate)

    console.log(result)
  });

  test("update_bulk_payment_record", async () => {
    // TODO: 
    // const result = await paymentHistoryApi.updateBulkPaymentRecord()
    // console.log(result)
  });

  test("update_payment_record", async () => {
    // TODO: 
  });
})
