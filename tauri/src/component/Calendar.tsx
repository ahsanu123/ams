import { useState, type JSX } from "react"
import { addMonths } from "date-fns"
import { getRandomArbitrary } from "../utility/random-arbitrary"
import "./Calendar.css"

const TOTAL_DAYS_TO_SHOW = 42
const DAY_TO_DISPLAY = 7

interface CalendarObject {
  year: number,
  month: number,
  date: number,
  totalDays: number,
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

function generateCalendarObject(currentDate: Date) {

  const year = currentDate.getFullYear()
  const month = currentDate.getMonth()
  const date = currentDate.getDate()
  const day = new Date(year, month, 1).getDay()

  const days = [
    "Minggu",
    "Senin",
    "Selasa",
    "Rabu",
    "Kamis",
    "Jumat",
    "Sabtu",
  ]

  const calendar: CalendarObject = {
    year,
    month,
    date,
    totalDays: new Date(year, month + 1, 0).getDate(),
  }

  const currentDay = days[day]
  const firstDateIndex = sundayStartdays.findIndex((item) => item === currentDay)
  for (let i = 0; i < TOTAL_DAYS_TO_SHOW; i++) {
    if (i < firstDateIndex)
      days.push("-")
    else if (i >= firstDateIndex + calendar.totalDays)
      days.push("-")
    else
      days.push((i - firstDateIndex + 1).toString())
  }

  return days
}

interface CalendarProps {
  showNavigator?: boolean
  title?: string,
  gridComponent?: (date: string) => JSX.Element
  onPrevMonthClicked?: (date: Date) => void
  onNextMonthClicked?: (date: Date) => void
}

export function Calendar(props: CalendarProps) {

  const {
    showNavigator = true,
    title,
    gridComponent,
    onPrevMonthClicked,
    onNextMonthClicked
  } = props

  const [date, setDate] = useState<Date>(new Date())
  const [days, setDays] = useState<string[]>(generateCalendarObject(date))

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

  const defaultGridComponent = (date: string, amount?: number | string) => {
    const includeDash = date.includes("-")
    return (
      <div
        className={`component ${includeDash ? "stripe" : ""}`}
      >
        <p>
          {(!includeDash && amount)
            ? `Ambil ${amount}` : "🚧"
          }
        </p>
        <sub>
          <b>
            {date}
          </b>
        </sub>
      </div>
    )
  }

  return (
    <>
      <h5>
        🌕
        {date.toLocaleDateString("id-id", { month: 'long' })}
        {" "}
        {date.toLocaleDateString("id-id", { year: 'numeric' })}
        {title ? ` - ${title}` : ""}
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
        {days.map((item, index) => (
          <>
            {index < DAY_TO_DISPLAY
              ? (<b>{item}</b>)
              : (gridComponent ? gridComponent(item) : defaultGridComponent(item, getRandomArbitrary(1, 4).toFixed()))
            }
          </>
        ))}

        <hr />
      </div>
    </>
  )
}
