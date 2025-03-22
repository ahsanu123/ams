import Calendar from "@/component/Calendar";
import EditDialogComponent from "@/component/EditDialog";
import { useEditPageStore } from "@/state";

export default function EditDataPage() {
  const isDialogvisible = useEditPageStore(state => state.isDialogVisible)

  return (
    <>
      <Calendar
        showNavigator
        adminMode
        title="✏️ Edit Record"
      />

      {isDialogvisible && (
        <EditDialogComponent />
      )}
    </>
  )
}
