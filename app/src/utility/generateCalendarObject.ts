import type { TakingRecordModel } from "@/api-models"
import { isSameDay, setDate } from "date-fns"

const TOTAL_DAYS_TO_SHOW = 42
const DAY_TO_DISPLAY = 7

export interface CalendarObject {
  year: number,
  month: number,
  date: number,
  totalDays: number,
}

export type CellType = 'HeaderLabel' | 'ShowDate' | 'HiddenDate'

export interface ICalendarCell {
  product?: TakingRecordModel,
  headerLabelText?: string,
  date?: Date,
  type: CellType,
}

export function generateCalendarObject(inputDate: Date, records: Array<TakingRecordModel>) {

  const currentyear = inputDate.getFullYear()
  const currentMonth = inputDate.getMonth()
  const currentDate = inputDate.getDate()

  const firstDayOfMonth = new Date(currentyear, currentMonth, 1).getDay()
  const totalDaysOfMonth = new Date(currentyear, currentMonth + 1, 0).getDate()

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
    totalDays: totalDaysOfMonth
  }

  // get day name from firstDayOfMonth
  const firstDayNameOfMonth = localDayName[firstDayOfMonth]
  // use day name to get first index
  const dayFromPrevMonth = localDayName.findIndex((item) => item === firstDayNameOfMonth)

  const calendarCellToShow = Array.from(Array(TOTAL_DAYS_TO_SHOW).keys())
  const isHiddenCell = (day: number) => (
    day < dayFromPrevMonth
    ||
    day >= (dayFromPrevMonth + calendarObject.totalDays)
  )
  const isCorrectDate = (date: number) => (date > 0 && date < calendarObject.totalDays)

  const calendarGrid = calendarCellToShow.map<ICalendarCell>((day) => {
    const date = day - dayFromPrevMonth + 1
    const currentDate = setDate(inputDate, date)

    const calendarCell: ICalendarCell = {
      type: isHiddenCell(day) ? 'HiddenDate' : 'ShowDate',
      product: isCorrectDate(date) ? records?.find((item) => isSameDay(item.takenDate, currentDate)) : undefined,
      date: currentDate,
    }
    return calendarCell
  });

  const topDayLabel = localDayName.map<ICalendarCell>((day) => ({
    type: 'HeaderLabel',
    headerLabelText: day
  }))

  calendarGrid.unshift(...topDayLabel);
  return calendarGrid
}
