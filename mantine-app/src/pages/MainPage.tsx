import { Customer } from "@/bindings/Customer";
import { RetrieveData } from "@/bindings/RetrieveData";
import BottomInformation from "@/components/BottomInformation";
import Calendar from "@/components/Calendar";
import RightSideBar from "@/components/RightSideBar";
import { Flex, Stack } from "@mantine/core";
import { useState } from "react";

export default function AdminPage() {
  const [month, setMonth] = useState(new Date())
  const customer: Customer = {
    customer_id: 0,
    customer_name: "Tresno",
    is_active: false,
    is_admin: false,
    created_date: new Date(),
    updated_date: new Date()
  }
  const retrievesData: RetrieveData[] = [
    {
      retrieve_data_id: 0,
      customer_id: 0,
      price_id: 1,
      amount: 12,
      date: new Date(2026, 5, 2),
      is_paid: true,
      customer: {
        customer_id: 0,
        customer_name: "Tresno",
        is_active: false,
        is_admin: false,
        created_date: new Date(),
        updated_date: new Date(),
      },
      price: {
        price_id: 0,
        date: new Date(),
        value: 11000
      }
    }
  ]

  return (
    <Calendar
      date={month}
      onNextMonth={(date) => setMonth(date)}
      onPrevMonth={(date) => setMonth(date)}
      onDateClick={(date) => console.log("onDateClick", date)}
      customer={customer}
      retrievesData={retrievesData}
    />
  )
}
