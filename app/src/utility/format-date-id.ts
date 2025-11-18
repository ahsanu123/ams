import { format } from "date-fns";
import { id } from "date-fns/locale";



export function formatDateId(date: Date, pattern: string = "PPPP") {
  return format(date, pattern, { locale: id })
}
