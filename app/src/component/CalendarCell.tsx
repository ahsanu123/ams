import type { ICalendarCell } from "@/utility";
import { format, isSameDay } from "date-fns";
import { id } from "date-fns/locale";
import { Text } from "@chakra-ui/react";
import './CalendarCell.css';

interface CalendarCellProps {
  data: ICalendarCell,
  isAdmin?: boolean,
  onCellClicked?: (date?: Date) => void
}

export default function CalendarCellComponent(props: CalendarCellProps) {

  const {
    data,
    onCellClicked,
    isAdmin = false,
  } = props

  const highlightCurrentDay = data.date && isSameDay(data.date, new Date())

  const headerLabelComponent = () => (
    <b>{data.headerLabelText}</b>
  )

  const dateComponent = () => (
    <>
      {data.product
        ? (
          <div
            onClick={() => isAdmin && onCellClicked?.(data.date)}
            className={`calendar-cell ${highlightCurrentDay ? 'highlight-current-day' : ''} ${isAdmin ? 'is-admin' : ''}`}
          >
            <div className='current-day'>

              <sub>
                {data.date && format(data.date, "dd", { locale: id })}
              </sub>
            </div>

            <div className='taking-amount'>
              <Text textStyle={'6xl'} fontWeight={'bold'}>
                {data.product?.amount}
              </Text>
            </div>
          </div>

        )
        : (
          <div
            onClick={() => isAdmin && onCellClicked?.(data.date)}
            className={`calendar-cell ${highlightCurrentDay ? 'highlight-current-day' : ''} ${isAdmin ? 'is-admin' : ''}`}
          >
            <div className='current-day'>
              <sub>
                {data.date && format(data.date, "dd", { locale: id })}
              </sub>
            </div>

          </div>
        )
      }
    </>
  )

  const hiddenDateComponent = () => (
    <div
      className="calendar-cell stripe"
    >
    </div>
  )

  switch (data.type) {
    case 'HeaderLabel':
      return headerLabelComponent()

    case 'ShowDate':
      return dateComponent()

    case 'HiddenDate':
      return hiddenDateComponent()

    default:
      return <></>
  }
}
