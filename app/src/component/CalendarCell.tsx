import type { Product } from "@/model";
import './CalendarCell.css'
import { formatAsRupiah } from "@/utility/format-as-rupiah";
import type { CellType } from "@/utility";

type CalendarCellType = 'stripe'

interface CalendarCellProps {
  data?: Product,
  type: CellType
}

export default function CalendarCellComponent(props: CalendarCellProps) {

  const {
    data,
    type
  } = props

  const headerLabelComponent = () => (
    <b>day</b>
  )

  const showDateComponent = () => (
    <div
      className="calendar-cell"
    >
      <div>
        <sub>
          💷 {formatAsRupiah(11000)}
        </sub>
      </div>

      <div
        className="center-cell-item"
      >
        <p>
          Ambil {data?.amount}
        </p>
      </div>

      <div>
        <sub>
          {new Date().toLocaleString("uk")}
        </sub>
      </div>
    </div>
  )

  const hiddenDateComponent = () => (
    <div
      className="calendar-cell stripe"
    >
      <h4>
        🚧
      </h4>
      <sub>
        No Data to Show
      </sub>
    </div>
  )

  switch (type) {
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
