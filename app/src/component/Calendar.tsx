import React, { useEffect } from "react"
import { addMonths } from "date-fns"
import CalendarCellComponent from "./CalendarCell"
import { useMainLayoutStore } from "@/state"
import { generateCalendarObject } from "@/utility"
import "./Calendar.css"

interface CalendarProps {
  showNavigator?: boolean
  title?: string,
  adminMode?: boolean,
  onPrevMonthClicked?: (date: Date) => void
  onNextMonthClicked?: (date: Date) => void
  onCellClicked?: (date?: Date) => void
}

export default function Calendar(props: CalendarProps) {

  const {
    showNavigator = false,
    adminMode = false,
    title,
    onPrevMonthClicked,
    onNextMonthClicked,
    onCellClicked
  } = props

  const userTakingRecords = useMainLayoutStore(state => state.userTakingRecords);
  const date = useMainLayoutStore(state => state.selectedMonth)
  const setDate = useMainLayoutStore(state => state.setSelectedDate)

  const calendarCells = useMainLayoutStore(state => state.calendarCells)
  const setCalendarCells = useMainLayoutStore(state => state.setCalendarCells)

  const lastSelectedUser = useMainLayoutStore(state => state.lastSelectedUser)

  const handleOnCurrentMonthClicked = () => {
    const newDate = new Date()

    setDate(newDate)
    setCalendarCells(generateCalendarObject(newDate, userTakingRecords))
  }

  const handleOnPrevMonthClicked = () => {
    const newDate = addMonths(date, -1)

    setDate(newDate)
    setCalendarCells(generateCalendarObject(newDate, userTakingRecords))
    onPrevMonthClicked?.(newDate)
  }

  const handleOnNextMonthClicked = () => {
    const newDate = addMonths(date, 1)

    setDate(newDate)
    setCalendarCells(generateCalendarObject(newDate, userTakingRecords))
    onNextMonthClicked?.(newDate)
  }

  const headerText = `🌕 ${date.toLocaleDateString("id-id", { month: 'long' })} ${date.toLocaleDateString("id-id", { year: 'numeric' })} ${title ? ` - ${title}` : ""}`

  useEffect(() => {
    setCalendarCells(generateCalendarObject(new Date(), userTakingRecords))
  }, [userTakingRecords])

  return (
    <>
      <div
        className="calendar-heading"
      >
        <div>
          <h5>
            {headerText}
          </h5>

          {
            showNavigator &&
            <>
              <button
                onClick={() => handleOnCurrentMonthClicked()}
              >
                <b>
                  Bulan Ini
                </b>
              </button>

              {" "}

              <button
                onClick={() => handleOnPrevMonthClicked()}
              >
                <b>
                  Bulan Lalu
                </b>
              </button>

              {" "}

              <button
                onClick={() => handleOnNextMonthClicked()}
              >
                <b>
                  Bulan Depan
                </b>
              </button>
            </>
          }
        </div>

        <div>
          <b>{lastSelectedUser?.username}</b>
        </div>
      </div>

      <div
        className="ams-calendar"
      >
        {calendarCells && calendarCells.map((cell, index) => (
          <React.Fragment
            key={index}
          >
            <CalendarCellComponent
              data={cell}
              onCellClicked={onCellClicked}
            />
          </React.Fragment>
        ))}
      </div>
    </>
  )
}
