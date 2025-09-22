import Calendar from "@/component/Calendar";
import EditDialogComponent from "@/component/EditDialog";
import { useEditPageStore, useMainLayoutStore } from "@/state";
import { useEffect } from "react";

export default function EditPickingRecordPage() {
  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)
  const isDialogvisible = useEditPageStore(state => state.isDialogVisible)
  const setIsDialogVisible = useEditPageStore(state => state.setDialogVisibility)

  useEffect(() => {
    setHeaderInformation({
      title: 'Edit Picking Data',
      description: 'you can edit and delete customer picking data'
    })
  }, [])

  const handleOnCellDateClicked = (date?: Date) => {
    setIsDialogVisible(true)
  }

  return (
    <>
      <main>
        <Calendar
          onCellClicked={(date) => handleOnCellDateClicked(date)}
        />
      </main>

      {isDialogvisible && (
        <EditDialogComponent />
      )}
    </>
  )
}
