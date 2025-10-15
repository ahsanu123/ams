import type { Product } from "@/model";
import { useEditPageStore, useMainLayoutStore } from "@/state";
import { formatAsRupiah } from "@/utility";
import { format } from "date-fns";
import { isSameDay } from "date-fns/fp";
import { id } from "date-fns/locale";
import "./EditDialog.css";
import VirtualKeypad from "./VirtualKeypad";


export default function EditDialogComponent() {

  const dialogData = useEditPageStore(state => state.dialogData)
  const setDialogData = useEditPageStore(state => state.setDialogData)

  const selectedUser = useMainLayoutStore(state => state.customer)

  const setDialogVisibility = useEditPageStore(state => state.setDialogVisibility)
  const isDialogVisible = useEditPageStore(state => state.isDialogVisible)

  const product = dialogData?.product
  const productPrice = formatAsRupiah((product?.amount ?? 0) * (product?.price ?? 0))

  const allProductOfThisMonth = useMainLayoutStore(state => state.allProductOfThisMonth)
  const productTakeByOtherUserForThisDate = useMainLayoutStore(state => state.allProductOfThisMonth).filter((item) => isSameDay(item.takenDate) && item.userId !== selectedUser?.id)
  console.log("productTakeByOtherUserForThisDate", productTakeByOtherUserForThisDate)

  const handleOnClose = () => {
    setDialogVisibility(!isDialogVisible)
  }

  const handleOnIsPaidChange = (paid: boolean) => {
    if (dialogData && selectedUser && dialogData.product) {
      const newProductData: Product = {
        ...dialogData.product,
        paid
      }

      setDialogData({
        date: dialogData.date,
        product: newProductData,
        user: selectedUser,
      })
    }
  }

  const handleOnUpdateData = () => {
    console.log("TODO update this data to backend", dialogData)
    setDialogVisibility(!isDialogVisible)
  }

  const handleOnAmountEdit = (value: number) => {
    if (dialogData && selectedUser && dialogData.product) {
      const newProductData: Product = {
        ...dialogData.product,
        amount: value
      }

      setDialogData({
        date: dialogData.date,
        product: newProductData,
        user: selectedUser,
      })
    }
  }

  return (
    <div
      className="dialog-background"
    >
      <dialog
        className="edit-dialog"
      >
        <div
          className="heading"
        >
          <b>Edit Data</b>

          {
            dialogData &&
            format(dialogData.date, "dd MMMM yyyy", { locale: id })
          }

          <button
            onClick={handleOnClose}
            className="button-transparent"
          >
            ❌
          </button>
        </div>
        <hr />

        <div
          className="dialog-content"
        >
          <div
            className="info-tab"
          >
            <div className="detail-tab">
              <h5>Select Customer</h5>
              <select>
                <option>User 1</option>
                <option>User 1</option>
                <option>User 1</option>
                <option>User 1</option>
              </select>
            </div>

            <div>
              <VirtualKeypad
                title="Edit Amount"
                defaultValue={product?.amount}
                confirmText="Update"
                handleOnConfirm={handleOnAmountEdit}
              />

            </div>

            <div>
              <button
                onClick={handleOnUpdateData}
              >
                📕 Update Data
              </button>
            </div>

          </div>
        </div>
      </dialog>
    </div>
  );

}
