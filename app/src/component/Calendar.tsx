import React, { useState, type JSX } from "react"
import { addMonths } from "date-fns"
import "./Calendar.css"
import type { ProductRecord } from "model"

const TOTAL_DAYS_TO_SHOW = 42
const DAY_TO_DISPLAY = 7

interface CalendarObject {
  year: number,
  month: number,
  date: number,
  totalDays: number,
}

type CellType = 'HeaderLabel' | 'ShowDate' | 'HiddenDate'

interface CalendarCellData {
  text: string,
  date?: number,
  take?: number
}

interface CalendarCell {
  data: CalendarCellData,
  type: CellType,
}

function generateCalendarObject(inputDate: Date) {

  const currentyear = inputDate.getFullYear()
  const currentMonth = inputDate.getMonth()
  const currentDate = inputDate.getDate()

  const dayInWeekIndex = new Date(currentyear, currentMonth, 1).getDay()
  const totalDaysOfThisMonth = new Date(currentyear, currentMonth + 1, 0).getDate()

  const localDayName = [
    "Minggu",
    "Senin",
    "Selasa",
    "Rabu",
    "Kamis",
    "Jumat",
    "Sabtu",
  ]

  const calendarObject: CalendarObject = {
    year: currentyear,
    month: currentMonth,
    date: currentDate,
    totalDays: totalDaysOfThisMonth
  }

  const currentDayName = localDayName[dayInWeekIndex]
  const dayFromPrevMonth = localDayName.findIndex((item) => item === currentDayName)

  const calendarCellToShow = Array.from(Array(TOTAL_DAYS_TO_SHOW).keys())
  const isHiddenCell = (day: number) => (
    day < dayFromPrevMonth
    ||
    day >= (dayFromPrevMonth + calendarObject.totalDays)
  )

  const calendarGrid = calendarCellToShow.map<CalendarCell>((day) => ({
    type: isHiddenCell(day) ? 'HiddenDate' : 'ShowDate',
    data: {
      text: isHiddenCell(day) ? "hide" : "show",
      date: isHiddenCell(day) ? undefined : day - dayFromPrevMonth + 1
    },
  }));

  const topDayLabel = localDayName.map<CalendarCell>((day) => ({
    type: 'HeaderLabel',
    data: {
      text: day
    }
  }))

  calendarGrid.unshift(...topDayLabel);
  return calendarGrid
}

interface CalendarProps {
  // TODO: Map recordData to cellCalendar, 
  recordData: ProductRecord,
  showNavigator?: boolean
  title?: string,
  gridComponent?: (date: string) => JSX.Element
  onPrevMonthClicked?: (date: Date) => void
  onNextMonthClicked?: (date: Date) => void
}

export default function Calendar(props: CalendarProps) {

  const {
    recordData,
    showNavigator = true,
    title,
    gridComponent,
    onPrevMonthClicked,
    onNextMonthClicked
  } = props

  const [date, setDate] = useState<Date>(new Date())
  const [calendarCells, setDays] = useState<CalendarCell[]>(generateCalendarObject(date))

  const handleOnPrevMonthClicked = () => {
    const newDate = addMonths(date, -1)

    setDate(newDate)
    setDays(generateCalendarObject(newDate))
    onPrevMonthClicked?.(newDate)
  }

  const handleOnNextMonthClicked = () => {
    const newDate = addMonths(date, 1)

    setDate(newDate)
    setDays(generateCalendarObject(newDate))
    onNextMonthClicked?.(newDate)
  }

  const gridCellView = (data: CalendarCell) => {
    return (
      <>
        {data.type === 'HeaderLabel' && (
          <b>{data.data.text}</b>
        )}

        {data.type === 'HiddenDate' && (
          <div
            className="component stripe"
          >
            <div
              className="item-cell"
            >
              <p>
                🚧
              </p>
            </div>
          </div>
        )}

        {data.type === 'ShowDate' && (
          <div
            className="component "
          >
            <div
              className="item-cell"
            >
              <p>
                Ambil
              </p>
              <b>
                2
              </b>
              <sub>
                tanggal {data.data.date}
              </sub>
            </div>
          </div>

        )}
      </>
    )

  }

  const headerText = `🌕 ${date.toLocaleDateString("id-id", { month: 'long' })} ${date.toLocaleDateString("id-id", { year: 'numeric' })} ${title ? ` - ${title}` : ""}`

  return (
    <>
      <h5>
        {headerText}
      </h5>
      {showNavigator && (
        <div>
          <button
            onClick={() => handleOnPrevMonthClicked()}
          >
            Previous Month
          </button>
          {" "}
          <button
            onClick={() => handleOnNextMonthClicked()}
          >
            Next Month
          </button>
        </div>
      )}

      <div
        className="ams-calendar"
      >
        {calendarCells.map((cell, index) => (
          <React.Fragment
            key={index}
          >
            {gridCellView(cell)}
          </React.Fragment>
        ))}
      </div>
    </>
  )
}
