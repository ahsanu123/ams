import { useGetAllRetrieveData } from "@/api/v1/retrieve-data-controller/retrieve-data-controller";
import Calendar from "@/components/Calendar";
import { useState } from "react";

export default function MainPage() {
  const [month, setMonth] = useState(new Date())
  const { data: retrievesData, status } = useGetAllRetrieveData({
    start_month: month.getMonth(),
    end_month: 12,
    year: 2026
  });

  return (
    <>
      {retrievesData && (
        <Calendar
          date={month}
          retrievesData={retrievesData}
        />
      )}
    </>
  )
}
