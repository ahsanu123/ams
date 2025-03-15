import React, { useState, type JSX } from "react"
import { addMonths } from "date-fns"
import "./Calendar.css"
import type { Product, ProductRecord } from "model"
import CalendarCellComponent from "./CalendarCell"
import { useProductStore } from "@/state"
import { generateCalendarObject, type ICalendarCell } from "@/utility"

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

  const date = useProductStore(state => state.selectedMonth)
  const setDate = useProductStore(state => state.setSelectedDate)

  const productCellDatas = useProductStore(state => state.productsCell)
  const setProductCell = useProductStore(state => state.setProductCell)

  const handleOnPrevMonthClicked = () => {
    const newDate = addMonths(date, -1)

    setDate(newDate)
    setProductCell(generateCalendarObject(newDate))
    onPrevMonthClicked?.(newDate)
  }

  const handleOnNextMonthClicked = () => {
    const newDate = addMonths(date, 1)

    setDate(newDate)
    setProductCell(generateCalendarObject(newDate))
    onNextMonthClicked?.(newDate)
  }

  const gridCellView = (data: ICalendarCell) => {
    return (
      <>
        <CalendarCellComponent
          type={data.type}
        />
      </>
    )
  }

  const headerText = `🌕 ${date.toLocaleDateString("id-id", { month: 'long' })} ${date.toLocaleDateString("id-id", { year: 'numeric' })} ${title ? ` - ${title}` : ""}`

  return (
    <>
      <h5>
        {headerText}
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
        {productCellDatas.map((cell, index) => (
          <React.Fragment
            key={index}
          >
            {gridCellView(cell)}
          </React.Fragment>
        ))}
      </div>
    </>
  )
}
