import { format, parseISO } from "date-fns";
import { id } from "date-fns/locale";

export function formatDateId(date: Date | string, pattern: string = "PPPPp") {
  if (typeof (date) === 'string') {
    return format(parseISO(date), pattern, { locale: id })
  }
  else {
    return format(date, pattern, { locale: id })
  }
}

