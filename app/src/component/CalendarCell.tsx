import './CalendarCell.css'
import type { ICalendarCell } from "@/utility";
import { format, isSameDay } from "date-fns";
import { id } from "date-fns/locale";
import { useMainLayoutStore } from '@/state';

interface CalendarCellProps {
  data: ICalendarCell,
  onCellClicked?: (date?: Date) => void
}

export default function CalendarCellComponent(props: CalendarCellProps) {

  const {
    data,
    onCellClicked
  } = props

  const isAdmin = useMainLayoutStore(state => state.isAdmin)

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
              <h2>
                {data.product?.amount}
              </h2>
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
      <h4>
        🚧
      </h4>
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
