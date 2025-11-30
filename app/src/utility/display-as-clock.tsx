import { formatDateId } from "./format-date-id"
import { Text } from "@chakra-ui/react"

export function displayAsClock(date: Date) {
  const dateClock = date.toLocaleTimeString("en-Us", {
    hour12: false
  })

  return (
    <Text fontWeight={'bold'}>
      {formatDateId(date, 'EEEE, dd MMMM yyyy')} - {dateClock}
    </Text>
  )
}
