import React, { useEffect, useState } from "react"
import { addMonths, format, isSameMonth } from "date-fns"
import CalendarCellComponent from "./CalendarCell"
import { useMainLayoutStore } from "@/state"
import { generateCalendarObject, type ICalendarCell } from "@/utility"
import { id } from "date-fns/locale"
import { formatAsRupiah } from "@/utility/format-as-rupiah"
import "./Calendar.css"

interface CalendarProps {
  showNavigator?: boolean
  title?: string,
  onPrevMonthClicked?: (date: Date) => void
  onNextMonthClicked?: (date: Date) => void
}

export default function Calendar(props: CalendarProps) {

  const {
    showNavigator = true,
    title,
    onPrevMonthClicked,
    onNextMonthClicked
  } = props

  const products = useMainLayoutStore(state => state.products);
  const date = useMainLayoutStore(state => state.selectedMonth)
  const setDate = useMainLayoutStore(state => state.setSelectedDate)

  const [productCellDatas, setProductCell] = useState<ICalendarCell[]>()

  const handleOnPrevMonthClicked = () => {
    const newDate = addMonths(date, -1)

    setDate(newDate)
    setProductCell(generateCalendarObject(newDate, products))
    onPrevMonthClicked?.(newDate)
  }

  const handleOnNextMonthClicked = () => {
    const newDate = addMonths(date, 1)

    setDate(newDate)
    setProductCell(generateCalendarObject(newDate, products))
    onNextMonthClicked?.(newDate)
  }

  const headerText = `🌕 ${date.toLocaleDateString("id-id", { month: 'long' })} ${date.toLocaleDateString("id-id", { year: 'numeric' })} ${title ? ` - ${title}` : ""}`

  const totalTake = products
    .filter((item) => isSameMonth(item.takenDate, date))
    .map((item) => item.amount)
    .reduce((a, b) => a + b, 0)

  const unpaidBill = products
    .filter((item) => isSameMonth(item.takenDate, date) && !item.paid)
    .map((item) => item.price * item.amount)
    .reduce((a, b) => a + b, 0)

  const paidBill = products
    .filter((item) => isSameMonth(item.takenDate, date) && item.paid)
    .map((item) => item.price * item.amount)
    .reduce((a, b) => a + b, 0)

  useEffect(() => {
    setProductCell(generateCalendarObject(new Date(), products))
  }, [products])

  return (
    <>
      <div
        className="calendar-heading"
      >
        <h5>
          {headerText}
        </h5>
        <select

        >
          <option>Pilih Nama Pengguna</option>
        </select>
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
        <sub>
          <b>
            {`Informasi Bulan ${format(date, "MMMM", { locale: id })} : `}
          </b>
          📍 Total Ambil <b>{totalTake}</b>, {" "}
          💷 Tagihan <b>{formatAsRupiah(unpaidBill)}</b>, {" "}
          ✅ Terbayar <b>{formatAsRupiah(paidBill)}</b>, {" "}
        </sub>
      </div>

      <div
        className="ams-calendar"
      >
        {productCellDatas && productCellDatas.map((cell, index) => (
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
