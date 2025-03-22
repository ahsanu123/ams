import './CalendarCell.css'
import { formatAsRupiah } from "@/utility/format-as-rupiah";
import type { ICalendarCell } from "@/utility";
import { format, isSameDay } from "date-fns";
import { id } from "date-fns/locale";
import { useEditPageStore, useMainLayoutStore } from '@/state';
import type { Product } from '@/model';

interface CalendarCellProps {
  data: ICalendarCell,
  adminMode?: boolean,
}

export default function CalendarCellComponent(props: CalendarCellProps) {

  const {
    data,
    adminMode = false,
  } = props

  const setDialogVisibility = useEditPageStore(state => state.setDialogVisibility);
  const isDialogVisible = useEditPageStore(state => state.isDialogVisible);
  const setDialogData = useEditPageStore(state => state.setDialogData);
  const productPrice = useMainLayoutStore(state => state.productPrice)

  const selectedUser = useMainLayoutStore(state => state.user)
  const isUserSelected = !!selectedUser

  const onDetailButtonClick = () => {
    // TODO:
    // get other user product from current date
    // const otherUserProduct = TODO

    if (selectedUser) {
      const emptyProductRecord: Product = {
        id: 0,
        userId: selectedUser.id,
        paid: false,
        productionDate: data.date!,
        takenDate: data.date!,
        price: productPrice,
        amount: 0,
        description: 'edited by admin'
      }

      setDialogData({
        product: data.product ?? emptyProductRecord,
        user: selectedUser,
        date: data.date!,
        otherData: []
      });
    }


    setDialogVisibility(!isDialogVisible)
  }

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
                {data.product.paid ? "✔️" : "💷"}
                {formatAsRupiah(data.product.price * data.product.amount)}
              </sub>
            </div>

            <div
              className="center-cell-item"
            >
              {adminMode
                ? (
                  <button
                    className="button-no-padding"
                    onClick={() => onDetailButtonClick()}
                    disabled={!isUserSelected}
                  >
                    🔧 Details
                  </button>
                ) :
                (
                  <p>
                    Ambil {data.product?.amount}
                  </p>
                )}
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
              {adminMode
                ? (
                  <button
                    className="button-no-padding"
                    onClick={() => onDetailButtonClick()}
                    disabled={!isUserSelected}
                  >
                    🔧 Details
                  </button>
                ) :
                (
                  <b>
                    🚫
                  </b>
                )}
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

    case 'ShowDate':
      return showDateComponent()

    case 'HiddenDate':
      return hiddenDateComponent()

    default:
      return <></>
  }
}
