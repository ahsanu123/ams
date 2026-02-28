import { RetrieveData } from "@/bindings/RetrieveData";
import {
  subMonths,
  endOfMonth,
  format,
  Locale,
  subDays,
  startOfMonth,
  eachDayOfInterval,
  startOfWeek,
  addDays,
  addMonths,
} from "date-fns";
import { da, id } from "date-fns/locale"

export enum Day {
  Sunday = 0,
  Monday = 1,
  Tuesday = 2,
  Wednesday = 3,
  Thursday = 4,
  Friday = 5,
  Saturday = 6
}

export const days = Object.entries({
  [0]: Day.Sunday,
  [1]: Day.Monday,
  [2]: Day.Tuesday,
  [3]: Day.Wednesday,
  [4]: Day.Thursday,
  [5]: Day.Friday,
  [6]: Day.Saturday,
}).map((day) => [Number(day[0]), day[1]])

export enum Month {
  January = 0,
  February = 1,
  March = 2,
  April = 3,
  May = 4,
  June = 5,
  July = 6,
  August = 7,
  September = 8,
  October = 9,
  November = 10,
  December = 11
}

export const months: [number, Month][] = Object.entries(
  {
    [0]: Month.January,
    [1]: Month.February,
    [2]: Month.March,
    [3]: Month.April,
    [4]: Month.May,
    [5]: Month.June,
    [6]: Month.July,
    [7]: Month.August,
    [8]: Month.September,
    [9]: Month.October,
    [10]: Month.November,
    [11]: Month.December,
  }
).map((month) => [Number(month[0]), month[1]])

export interface ICalendarDateObject {
  date: Date,
  day: Day,
  retrieveData?: RetrieveData
}

export interface ICalendarObject {
  year: number,
  month: Month,
  monthStr: string,
  staticDays: string[],

  dates: ICalendarDateObject[]
}

function getLocalizedWeekdays(
  weekStartsOn: 0 | 1 | 2 | 3 | 4 | 5 | 6 = 0,
  locale: Locale = id
) {
  const start = startOfWeek(new Date(), { weekStartsOn });
  const days: string[] = [];

  for (let i = 0; i < 7; i++) {
    days.push(format(addDays(start, i), "EEEE", { locale: locale }));
  }

  return days;
}

const staticWeekDays = getLocalizedWeekdays();

export function localizedDayAsStr(date: Date, locale: Locale = id): string {
  return format(date, "EEEE", { locale: locale });
}

export function generateCalendarObject(date?: Date, local?: Locale): ICalendarObject {
  const currentMonth = date ?? new Date();
  const previousMonth = subMonths(currentMonth, 1);

  const currentMonthDate = eachDayOfInterval({
    start: startOfMonth(currentMonth),
    end: endOfMonth(currentMonth)
  })

  const year = currentMonth.getFullYear();
  const month = currentMonth.getMonth() as Month;
  const month_str = format(currentMonth, "MMMM", { locale: local ?? id });

  const lastDayOfLastMonth = endOfMonth(previousMonth).getDay() as Day;
  const lastDayOfCurrentMonth = endOfMonth(currentMonth).getDay() as Day;

  let lastMonthDateUntilCurrentMonthFirstDate: Date[] = [];
  let nextMonthAfterCurrentMonthLastDate: Date[] = [];

  if (lastDayOfLastMonth !== Day.Saturday) {
    const lastMonthDate = endOfMonth(previousMonth);
    for (let lastDay = 0; lastDay <= lastDayOfLastMonth; lastDay++) {
      const date = subDays(lastMonthDate, lastDay)
      lastMonthDateUntilCurrentMonthFirstDate.push(date);
    }
    lastMonthDateUntilCurrentMonthFirstDate = lastMonthDateUntilCurrentMonthFirstDate.reverse()
  }

  if (lastDayOfCurrentMonth !== Day.Saturday) {
    const nextMonth = startOfMonth(addMonths(currentMonth, 1));
    for (let currentDay = 0; currentDay < Day.Saturday - lastDayOfCurrentMonth; currentDay++) {
      const date = addDays(nextMonth, currentDay);
      nextMonthAfterCurrentMonthLastDate.push(date);
    }
  }

  let dates: ICalendarDateObject[] = [
    ...lastMonthDateUntilCurrentMonthFirstDate.map((date) => ({ date, day: date.getDay() as Day })),
    ...currentMonthDate.map((date) => ({ date, day: date.getDay() as Day })),
    ...nextMonthAfterCurrentMonthLastDate.map((date) => ({ date, day: date.getDay() as Day }))
  ];


  return {
    year,
    month,
    monthStr: month_str,
    staticDays: staticWeekDays,
    dates,
  }
}

function genDateKey(date: Date): string {
  return `${date.getFullYear()}-${date.getMonth()}-${date.getDate()}`
}

export function generateCalendarObjectWithRetrieveData(retrievesData?: RetrieveData[], date?: Date, local?: Locale): ICalendarObject {
  const objs = generateCalendarObject(date, local)

  const dataByDayMap = new Map(
    retrievesData?.filter((obj) => obj.date !== undefined).map((data) => [genDateKey(data.date), data])
  );

  objs.dates = objs.dates.map((obj) => ({
    ...obj,
    retrieveData: dataByDayMap.get(genDateKey(obj.date))
  }));

  return objs;
}
