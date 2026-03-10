import { formatDateId } from "@/utilities/format-date-id"
import { Flex, Text } from "@mantine/core"
import { useEffect, useState } from "react"
import { TbClockHour10 } from "react-icons/tb";

const displayAsClock = (date: Date) => {
  const dateClock = date.toLocaleTimeString("en-Us", {
    hour12: false
  })

  return (
    {
      dateStr: formatDateId(date, 'dd MMMM yyyy'),
      timeStr: dateClock
    }
  )
}

export default function Clock() {
  const [date, setDate] = useState<Date>(new Date())

  const displayDate = displayAsClock(date);

  useEffect(() => {
    const timer = setInterval(() => setDate(new Date()), 1000)
    return () => clearInterval(timer)
  }, [])

  return (
    <Flex
      p={"0 20px"}
      justify={'space-between'}
      align={'center'}
    >
      <TbClockHour10 size={'50px'} />
      <Text
        fw={'bold'}
        size="35px"
      >
        {displayDate.timeStr}
      </Text>
      <Text
        fw={'bold'}
        size="20px"
      >
        {displayDate.dateStr}
      </Text>
    </Flex>
  )
}
