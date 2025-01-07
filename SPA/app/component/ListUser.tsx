import GridButton from "./GridButton"

interface ListUserProps {
  users: string[],
  handleOnUserSelected: (username: string) => void
}

export default function ListUser(props: ListUserProps) {

  const {
    users,
    handleOnUserSelected
  } = props

  // change to use loader 

  return (
    <>
      <GridButton
        buttons={users}
        onClick={handleOnUserSelected}
      />
    </>
  )
}
