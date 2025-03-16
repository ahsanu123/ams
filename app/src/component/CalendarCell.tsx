import './CalendarCell.css'
import { formatAsRupiah } from "@/utility/format-as-rupiah";
import type { ICalendarCell } from "@/utility";
import { format, isSameDay } from "date-fns";
import { id } from "date-fns/locale";

interface CalendarCellProps {
  data: ICalendarCell,
}

export default function CalendarCellComponent(props: CalendarCellProps) {

  const {
    data,
  } = props

  const highlightCurrentDay = data.date && isSameDay(data.date, new Date())

  const headerLabelComponent = () => (
    <b>{data.headerLabelText}</b>
  )

  const showDateComponent = () => (
    <>
      {data.product
        ? (
          <div
            className={`calendar-cell ${highlightCurrentDay ? 'highlight-current-day' : ''}`}
          >
            <div>
              <sub>
                {data.product.paid ? "✅" : "💷"}
                {formatAsRupiah(data.product.price * data.product.amount)}
              </sub>
            </div>

            <div
              className="center-cell-item"
            >
              <p>
                Ambil {data.product?.amount}
              </p>
            </div>

            <div>
              <sub>
                {data.date && format(data.date, "dd MMMM yyyy", { locale: id })}
              </sub>
            </div>
          </div>

        )
        : (
          <div
            className={`calendar-cell ${highlightCurrentDay ? 'highlight-current-day' : ''}`}
          >
            <div>
              <sub>
                <b>Tidak Ambil</b>
              </sub>
            </div>

            <div>
              <b>
                0️⃣
              </b>
            </div>

            <div>
              <sub>
                {data.date && format(data.date, "dd MMMM yyyy", { locale: id })}
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
      <sub>
        Disembunyikan
      </sub>
    </div>
  )

  switch (data.type) {
    case 'HeaderLabel':
      return headerLabelComponent()
      break;

    case 'ShowDate':
      return showDateComponent()
      break;

    case 'HiddenDate':
      return hiddenDateComponent()
      break;

    default:
      return <></>
      break;
  }
}
