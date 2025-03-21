import { useEditPageStore } from "@/state";
import "./EditDialog.css"
import { format } from "date-fns";
import { id } from "date-fns/locale";

interface EditDialogProps {
  show?: boolean
}

export default function EditDialogComponent(props: EditDialogProps) {

  const {
    show
  } = props

  const dialogData = useEditPageStore(state => state.dialogData)
  const setDialogVisibility = useEditPageStore(state => state.setDialogVisibility)
  const isDialogVisible = useEditPageStore(state => state.isDialogVisible)

  const handleOnClose = () => {
    setDialogVisibility(!isDialogVisible)
  }

  return (
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
          <label>
            <p>
              Username
            </p>
            <input
              value="paijo"
              disabled
            />
          </label>

        </div>

        <div
          className="detail-tab"
        >
          hello world
        </div>
      </div>
    </dialog>
  );

}
