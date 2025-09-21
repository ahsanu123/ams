import React, { useEffect } from "react"
import { addMonths } from "date-fns"
import CalendarCellComponent from "./CalendarCell"
import { useMainLayoutStore } from "@/state"
import { generateCalendarObject } from "@/utility"
import "./Calendar.css"

interface CalendarProps {
  showNavigator?: boolean
  title?: string,
  adminMode?: boolean,
  onPrevMonthClicked?: (date: Date) => void
  onNextMonthClicked?: (date: Date) => void
}

export default function Calendar(props: CalendarProps) {

  const {
    showNavigator,
    adminMode = false,
    title,
    onPrevMonthClicked,
    onNextMonthClicked,
  } = props

  const products = useMainLayoutStore(state => state.products);
  const setProducts = useMainLayoutStore(state => state.setProducts)
  const date = useMainLayoutStore(state => state.selectedMonth)
  const setDate = useMainLayoutStore(state => state.setSelectedDate)

  const calendarCells = useMainLayoutStore(state => state.calendarCells)
  const setCalendarCells = useMainLayoutStore(state => state.setCalendarCells)


  const handleOnPrevMonthClicked = () => {
    const newDate = addMonths(date, -1)

    setDate(newDate)
    setCalendarCells(generateCalendarObject(newDate, products))
    onPrevMonthClicked?.(newDate)
  }

  const handleOnNextMonthClicked = () => {
    const newDate = addMonths(date, 1)

    setDate(newDate)
    setCalendarCells(generateCalendarObject(newDate, products))
    onNextMonthClicked?.(newDate)
  }

  const headerText = `🌕 ${date.toLocaleDateString("id-id", { month: 'long' })} ${date.toLocaleDateString("id-id", { year: 'numeric' })} ${title ? ` - ${title}` : ""}`

  useEffect(() => {
    setCalendarCells(generateCalendarObject(new Date(), products))
  }, [products])

  return (
    <>
      <div
        className="calendar-heading"
      >
        <h5>
          {headerText}
        </h5>
      </div>
      {showNavigator && (
        <div>
          <button
            onClick={() => handleOnPrevMonthClicked()}
          >
            Bulan Sebelumnya
          </button>

          {" "}

          <button
            onClick={() => handleOnNextMonthClicked()}
          >
            Bulan Selanjutnya
          </button>
        </div>
      )}

      <div
        className="user-information"
      >
        {/* <sub> */}
        {/*   <b> */}
        {/*     {`Informasi Bulan ${format(date, "MMMM", { locale: id })} : `} */}
        {/*   </b> */}
        {/*   📍 Total Ambil <b>{totalTake}</b>, {" "} */}
        {/*   💷 Tagihan <b>{formatAsRupiah(unpaidBill)}</b>, {" "} */}
        {/*   ✔️ Terbayar <b>{formatAsRupiah(paidBill)}</b>, {" "} */}
        {/* </sub> */}
      </div>

      <div
        className="ams-calendar"
      >
        {calendarCells && calendarCells.map((cell, index) => (
          <React.Fragment
            key={index}
          >
            <CalendarCellComponent
              data={cell}
            />
          </React.Fragment>
        ))}
      </div>
    </>
  )
}
