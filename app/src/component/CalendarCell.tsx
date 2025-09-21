import './CalendarCell.css'
import { formatAsRupiah } from "@/utility/format-as-rupiah";
import type { ICalendarCell } from "@/utility";
import { format, isSameDay } from "date-fns";
import { id } from "date-fns/locale";
import { useEditPageStore, useMainLayoutStore } from '@/state';
import type { Product } from '@/model';

interface CalendarCellProps {
  data: ICalendarCell,
}

export default function CalendarCellComponent(props: CalendarCellProps) {

  const {
    data,
  } = props

  const isAdmin = useMainLayoutStore(state => state.isAdmin)
  const setDialogVisibility = useEditPageStore(state => state.setDialogVisibility);
  const isDialogVisible = useEditPageStore(state => state.isDialogVisible);
  const setDialogData = useEditPageStore(state => state.setDialogData);
  const productPrice = useMainLayoutStore(state => state.productPrice)

  const selectedUser = useMainLayoutStore(state => state.user)
  const isUserSelected = !!selectedUser

  // const onDetailButtonClick = () => {
  //
  //   if (selectedUser) {
  //     const emptyProductRecord: Product = {
  //       id: 0,
  //       userId: selectedUser.id,
  //       paid: false,
  //       productionDate: data.date!,
  //       takenDate: data.date!,
  //       price: productPrice,
  //       amount: 0,
  //       description: 'edited by admin'
  //     }
  //
  //     setDialogData({
  //       product: data.product ?? emptyProductRecord,
  //       user: selectedUser,
  //       date: data.date!,
  //     });
  //   }
  //
  //
  //   setDialogVisibility(!isDialogVisible)
  // }

  const handleOnCellClicked = () => {
    console.log("iam admin")

  }
  const highlightCurrentDay = data.date && isSameDay(data.date, new Date())

  const headerLabelComponent = () => (
    <b>{data.headerLabelText}</b>
  )

  const dateComponent = () => (
    <>
      {data.product
        ? (
          <div
            onClick={() => isAdmin && handleOnCellClicked()}
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
            onClick={() => isAdmin && handleOnCellClicked()}
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
