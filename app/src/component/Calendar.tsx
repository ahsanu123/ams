import type { TakingRecordModel, UserModel } from "@/api-models"
import { generateCalendarObject, type ICalendarCell } from "@/utility"
import { Box, Button, Flex, Heading, Text } from "@chakra-ui/react"
import { addMonths } from "date-fns"
import React, { useEffect, useState } from "react"
import CalendarCellComponent from "./CalendarCell"
import "./Calendar.css"
import { AiOutlineCaretLeft, AiOutlineCaretRight, AiOutlineEnvironment } from "react-icons/ai"

interface CalendarProps {
  takingRecords?: TakingRecordModel[],
  user?: UserModel,
  isAdmin?: boolean,

  showNavigator?: boolean
  title?: string,

  onPrevMonthClicked?: (date: Date) => void
  onNextMonthClicked?: (date: Date) => void
  onCellClicked?: (date?: Date) => void
}

export default function Calendar(props: CalendarProps) {

  const {
    takingRecords,
    user,
    isAdmin = false,
    showNavigator = false,
    title,
    onPrevMonthClicked,
    onNextMonthClicked,
    onCellClicked
  } = props

  const [calendarCells, setCalendarCells] = useState<ICalendarCell[]>(generateCalendarObject(new Date(), takingRecords ?? []));
  const [date, setDate] = useState<Date>(new Date());

  const handleOnCurrentMonthClicked = () => {
    const newDate = new Date()

    setDate?.(newDate)
    setCalendarCells(generateCalendarObject(newDate, takingRecords ?? []))
  }

  const handleOnPrevMonthClicked = () => {
    const newDate = addMonths(date, -1)

    setDate?.(newDate)
    setCalendarCells(generateCalendarObject(newDate, takingRecords ?? []))
    onPrevMonthClicked?.(newDate)
  }

  const handleOnNextMonthClicked = () => {
    const newDate = addMonths(date, 1)

    setDate(newDate)
    setCalendarCells(generateCalendarObject(newDate, takingRecords ?? []))
    onNextMonthClicked?.(newDate)
  }

  useEffect(() => {
    setCalendarCells(generateCalendarObject(new Date(), takingRecords ?? []));
  }, [takingRecords])

  const headerText = `🌕 ${date.toLocaleDateString("id-id", { month: 'long' })} ${date.toLocaleDateString("id-id", { year: 'numeric' })} ${title ? ` - ${title}` : ""}`

  return (
    <Box className="calendar">
      <Flex className="calendar-flex">
        <Heading minWidth={200}>
          {headerText}
        </Heading>

        {
          showNavigator && (
            <Flex className="calendar-navigation">
              <Button
                onClick={() => handleOnCurrentMonthClicked()}
              >
                <AiOutlineEnvironment />
                Bulan Ini
              </Button>

              <Button
                onClick={() => handleOnPrevMonthClicked()}
              >
                <AiOutlineCaretLeft />
                Bulan Lalu
              </Button>

              <Button
                onClick={() => handleOnNextMonthClicked()}
              >
                <AiOutlineCaretRight />
                Bulan Depan
              </Button>
            </Flex>
          )
        }
        <Text textStyle={'md'}>{user?.username}</Text>
      </Flex>

      <Box
        className="ams-calendar"
      >
        {calendarCells && calendarCells.map((cell, index) => (
          <React.Fragment
            key={index}
          >
            <CalendarCellComponent
              data={cell}
              onCellClicked={onCellClicked}
              isAdmin={isAdmin}
            />
          </React.Fragment>
        ))}
      </Box>
    </Box>
  )
}
