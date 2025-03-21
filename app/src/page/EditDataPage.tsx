import Calendar from "@/component/Calendar";
import EditDialogComponent from "@/component/EditDialog";
import { useEditPageStore } from "@/state";

export default function EditDataPage() {
  const isDialogvisible = useEditPageStore(state => state.isDialogVisible)
  return (
    <>
      <h2>Edit Data Page</h2>
      <b>For Admin Only</b>

      <Calendar
        showNavigator
        adminMode
        onNextMonthClicked={(date) => undefined}
        onPrevMonthClicked={(date) => undefined}
      />

      {isDialogvisible && (
        <EditDialogComponent />
      )}
    </>
  )
}
