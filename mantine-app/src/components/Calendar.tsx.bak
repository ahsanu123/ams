import type { TakingRecordModel, UserModel } from "@/api-models"
import { takingRecordCommand } from "@/commands"
import { formatDateId, generateCalendarObject, type ICalendarCell } from "@/utility"
import { Avatar, Box, Button, Flex, Heading, Text } from "@chakra-ui/react"
import { addMonths } from "date-fns"
import React, { useEffect, useState } from "react"
import { AiFillCaretLeft, AiFillCaretRight, AiFillMoon, AiOutlineCaretLeft, AiOutlineCaretRight, AiOutlineEnvironment } from "react-icons/ai"
import { userManagementCommand } from '../commands/user-management-api'
import CalendarCellComponent from "./CalendarCell"
import "./Calendar.css"

interface CalendarProps {
  takingRecords?: TakingRecordModel[],
  user?: UserModel,
  isAdmin?: boolean,
  customerMode?: boolean,
  month?: Date,
  hideCustomerAvatar?: boolean,

  showNavigator?: boolean
  title?: string,

  onPrevMonthClicked?: (date: Date) => void
  onNextMonthClicked?: (date: Date) => void
  onCellClicked?: (date?: Date) => void

  onCustomerIdChange?: (id: number) => void
}

export default function Calendar(props: CalendarProps) {

  const {
    takingRecords,
    user,
    isAdmin = false,
    customerMode = false,
    showNavigator = false,
    hideCustomerAvatar = false,
    month,
    title,
    onCustomerIdChange,
    onPrevMonthClicked,
    onNextMonthClicked,
    onCellClicked,
  } = props


  const [calendarCells, setCalendarCells] = useState<ICalendarCell[]>(generateCalendarObject(new Date(), takingRecords ?? []));
  const [date, setDate] = useState<Date>(new Date());
  const [listCustomer, setListCustomer] = useState<UserModel[]>([])
  const [selectedCustomer, setSelectedCustomer] = useState<UserModel | undefined>(user)
  const [internalTakingRecord, setInternalTakingRecord] = useState<TakingRecordModel[]>(takingRecords ?? [])

  const handleOnCurrentMonthClicked = () => {
    const newDate = new Date()

    setDate?.(newDate)
    setCalendarCells(generateCalendarObject(newDate, internalTakingRecord))
  }

  const handleOnPrevMonthClicked = () => {
    const newDate = addMonths(date, -1)

    setDate?.(newDate)
    setCalendarCells(generateCalendarObject(newDate, internalTakingRecord))
    onPrevMonthClicked?.(newDate)
  }

  const handleOnNextMonthClicked = () => {
    const newDate = addMonths(date, 1)

    setDate(newDate)
    setCalendarCells(generateCalendarObject(newDate, internalTakingRecord))
    onNextMonthClicked?.(newDate)
  }

  const handleOnPrevCustomer = () => {
    if (selectedCustomer === undefined) return

    let currentIndex = listCustomer.findIndex(pr => pr.id === selectedCustomer.id)
    if (currentIndex <= 0) {
      const customer = listCustomer[listCustomer.length - 1]

      setSelectedCustomer(customer)
      onCustomerIdChange?.(customer.id!)
    }
    else {
      const customer = listCustomer[currentIndex - 1]

      setSelectedCustomer(customer)
      onCustomerIdChange?.(customer.id!)
    }
  }
  const handleOnNextCustomer = () => {
    if (selectedCustomer === undefined) return

    let currentIndex = listCustomer.findIndex(pr => pr.id === selectedCustomer.id)
    if (currentIndex >= listCustomer.length - 1) {
      const customer = listCustomer[0]
      setSelectedCustomer(customer)
      onCustomerIdChange?.(customer.id!)
    }
    else {
      const customer = listCustomer[currentIndex + 1]
      setSelectedCustomer(customer)
      onCustomerIdChange?.(customer.id!)
    }
  }

  useEffect(() => {
    setCalendarCells(generateCalendarObject(new Date(), takingRecords ?? []));

    if (takingRecords)
      setInternalTakingRecord(takingRecords)
  }, [takingRecords])

  useEffect(() => {
    userManagementCommand.getAllActiveUser()
      .then((customers) => {
        setListCustomer(customers)

        if (customers.length > 0 && user === undefined) {
          const customer = customers[0]
          setSelectedCustomer(customer)
          onCustomerIdChange?.(customer.id!)
        }
      });
  }, [])

  useEffect(() => {
    setSelectedCustomer(user)
    if (user)
      onCustomerIdChange?.(user.id)

    if (user && month) {
      setDate(month)
      takingRecordCommand.getTakingRecordByUserIdAndMonth(user.id!, month)
        .then((value) => {
          setInternalTakingRecord(value)
          setCalendarCells(generateCalendarObject(month, value))
        }
        )
    }

  }, [user, month])

  const headerText = `${formatDateId(date, "MMMM yyyy")} 
                      ${title ? ` - ${title}` : ""}`

  return (
    <Box className="calendar">
      <Flex className="calendar-flex">
        <Heading minWidth={250} size={'2xl'}>
          <Flex alignItems={'center'} gap={'15px'}>
            <AiFillMoon size={'36px'} color='orange' />
            {headerText}
          </Flex>
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

        {
          customerMode && (
            <Flex gap={5}>
              <Button
                onClick={() => handleOnPrevCustomer()}
              >
                <AiFillCaretLeft />
                Sebelumnya
              </Button>
              <Button
                onClick={() => handleOnNextCustomer()}
              >
                Selanjutnya
                <AiFillCaretRight />
              </Button>
            </Flex>
          )
        }

        {
          selectedCustomer && !hideCustomerAvatar &&
          <React.Fragment>
            <Avatar.Root>
              <Avatar.Fallback name={selectedCustomer.username} />
            </Avatar.Root>
            <Text textStyle={'4xl'} fontWeight={'bold'}>
              {selectedCustomer.username}
            </Text>
          </React.Fragment>
        }
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
    </Box >
  )
}
