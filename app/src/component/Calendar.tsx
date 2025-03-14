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

type DisplayCalendarType = 'HeaderLabel' | 'ShowDate' | 'HiddenDate'

interface CalendarItemCellData {
  text: string,
  date?: number,
  take?: number
}

interface CalendarItemCell {
  data: CalendarItemCellData,
  type: DisplayCalendarType,
}

const sundayStartdays = [
  "Minggu",
  "Senin",
  "Selasa",
  "Rabu",
  "Kamis",
  "Jumat",
  "Sabtu",
]

// TODO: refactor variable inside this function to use more makesense name
function generateCalendarObject(currentDate: Date) {

  const year = currentDate.getFullYear()
  const month = currentDate.getMonth()
  const date = currentDate.getDate()
  const dayInWeekIndex = new Date(year, month, 1).getDay()

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
    year,
    month,
    date,
    totalDays: new Date(year, month + 1, 0).getDate(),
  }

  const currentDay = localDayName[dayInWeekIndex]
  const firstSpanDate = sundayStartdays.findIndex((item) => item === currentDay)

  const calendarCellToShow = Array.from(Array(TOTAL_DAYS_TO_SHOW).keys())
  const isHiddenCell = (day: number) => (day < firstSpanDate || day >= (firstSpanDate + calendarObject.totalDays))

  const calendarCells = calendarCellToShow.map<CalendarItemCell>((day) => ({
    type: isHiddenCell(day) ? 'HiddenDate' : 'ShowDate',
    data: {
      text: isHiddenCell(day) ? "hide" : "show",
      date: isHiddenCell(day) ? undefined : day - firstSpanDate + 1
    },
  }));

  const dayName = localDayName.map<CalendarItemCell>((day) => ({
    type: 'HeaderLabel',
    data: {
      text: day
    }
  }))

  calendarCells.unshift(...dayName);
  return calendarCells
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
  const [calendarCells, setDays] = useState<CalendarItemCell[]>(generateCalendarObject(date))

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

  const gridCellView = (data: CalendarItemCell) => {
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
