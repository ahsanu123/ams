import { parseISO } from "date-fns"

export function utilToDate(date: Date) {
  return parseISO(date.toString())
}
