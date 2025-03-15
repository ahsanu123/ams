import type { Product } from "@/model"

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
  product?: Product,
  type: CellType,
}

export function generateCalendarObject(inputDate: Date) {

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

  const calendarGrid = calendarCellToShow.map<ICalendarCell>((day) => ({
    type: isHiddenCell(day) ? 'HiddenDate' : 'ShowDate',
  }));

  const topDayLabel = localDayName.map<ICalendarCell>((day) => ({
    type: 'HeaderLabel',
  }))

  calendarGrid.unshift(...topDayLabel);
  return calendarGrid
}
