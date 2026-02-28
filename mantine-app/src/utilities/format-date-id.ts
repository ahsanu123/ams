import { format } from "date-fns";
import { id } from "date-fns/locale";

export function formatDateId(date: Date, pattern: string = "PPPPp") {
  return format(date, pattern, { locale: id })
}
