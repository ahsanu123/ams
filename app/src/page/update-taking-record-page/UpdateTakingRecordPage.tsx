import Calendar from "@/component/Calendar"
import Scroller from "@/component/Scroller"
import VirtualKeypad from "@/component/VirtualKeypad"
import { Text, Box, Button, CloseButton, Drawer, Flex, Portal, Stack, Avatar, DataList, Table, Badge } from "@chakra-ui/react"
import { useEffect, useState } from "react"
import { AiFillCloseCircle, AiFillEdit, AiOutlinePlusSquare, AiTwotoneCalendar } from "react-icons/ai"
import { useUpdateTakingPageState } from "./update-taking-record-page-state"
import { takingRecordCommand, userManagementCommand } from "@/commands"
import type { Route } from "./+types/UpdateTakingRecordPage"
import { dataListItemValue, formatDateId, toaster } from "@/utility"
import { format } from "date-fns"
import React from "react"
import type { TakingRecordModel, UserModel } from "@/api-models"
import { useRevalidator } from "react-router"

enum ListActionEnum {
  GetPageModel = 'GetPageModel',
  MakePayment = 'MakePayment'
}

interface IFetcherActionResult {
}

interface IGetPageModelClientRequest {
  userId: number,
  date: Date,
  _action: ListActionEnum,
}

export async function clientLoader() {
  const activeCustomer = await userManagementCommand.getAllActiveUser()
  return {
    activeCustomer
  }
}

export async function clientAction() {

}


export default function UpdateTakingRecordPage({
  loaderData
}: Route.ComponentProps) {

  const { revalidate } = useRevalidator()

  const customers = useUpdateTakingPageState(state => state.customers)
  const setCustomers = useUpdateTakingPageState(state => state.setCustomers)

  const selectedDate = useUpdateTakingPageState(state => state.selectedDate)
  const setSelectedDate = useUpdateTakingPageState(state => state.setSelectedDate)

  const takingRecords = useUpdateTakingPageState(state => state.takingRecords)
  const setTakingRecords = useUpdateTakingPageState(state => state.setTakingRecord)

  const selectedTakingRecord = useUpdateTakingPageState(state => state.selectedTakingRecord)
  const setSelectedTakingRecord = useUpdateTakingPageState(state => state.setSelectedTakingRecord)

  const isAddingNewRecord = useUpdateTakingPageState(state => state.isAddingNewRecord)
  const setIsAddingAddNewRecord = useUpdateTakingPageState(state => state.setIsAddingAddNewRecord)

  const selectedCustomer = useUpdateTakingPageState(state => state.selectedCustomer)
  const setSelectedCustomer = useUpdateTakingPageState(state => state.setSelectedCustomer)

  const [open, setOpen] = useState(false)

  const handleOnCalendarCellClicked = (date: Date | undefined) => {
    setOpen(true)
    setSelectedDate(date)
  }

  const loadTakingRecords = (selectedDate?: Date) => {
    if (selectedDate)
      takingRecordCommand.getTakingRecordByDay(selectedDate)
        .then((records) => setTakingRecords(records))
  }

  const handleOnEditTakingRecordButtonClick = (record: TakingRecordModel) => {
    const customer = customers.find(pr => pr.id! === record.userId)

    if (!customer) {
      toaster.create({
        title: `Customer With ID=${record.userId} is not found `,
        type: 'error'
      });
      return;
    }

    setSelectedCustomer(customer)
    setSelectedTakingRecord(record)
  }

  const handleOnUpdateTakingRecord = (amount: number) => {
    if (selectedTakingRecord)
      takingRecordCommand.upsertTakingRecord({
        ...selectedTakingRecord,
        amount
      }).then(value => {
        toaster.create({
          title: `Success Updating, ${value}`,
          type: 'success'
        })
        setSelectedTakingRecord(undefined)

        // FIXME: Find Better way to to this
        loadTakingRecords(selectedDate)
      }).catch(reason => {
        toaster.create({
          title: `Success Updating, ${reason}`,
          type: 'error'
        })
      })
  }

  const handleOnAddRecord = (amount: number) => {
    if (!selectedCustomer || !selectedDate) return
    takingRecordCommand.addNewTakingRecordByDate(selectedCustomer.id!, amount, selectedDate)
      .then(value => {
        toaster.create({
          title: `${selectedCustomer.username} Pick ${amount} Ampas`,
          type: 'success'
        })
        setIsAddingAddNewRecord(false)

        // FIXME: Find Better way to to this
        loadTakingRecords(selectedDate)
      }).catch(reason => {
        toaster.create({
          title: `Success Updating, ${reason}`,
          type: 'error'
        })
      })

  }

  const valueMustBeNonZero = (value: number) => {
    if (value == 0) return "Tidak Boleh Nol (0)"
    return undefined
  }

  const handleOnDrawerClosed = () => {
    setTakingRecords([])
    setSelectedCustomer(undefined)
    setSelectedDate(undefined)
    setSelectedTakingRecord(undefined)
    setIsAddingAddNewRecord(false)
  }

  const renderVirtualKeyboard = () =>
    <>
      {isAddingNewRecord
        ?
        <VirtualKeypad
          title={"Tambahkan Record"}
          defaultValue={0}
          handleOnConfirm={handleOnAddRecord}
          validatorFunction={valueMustBeNonZero}
        />
        :
        <VirtualKeypad
          title={selectedTakingRecord
            ? `${selectedCustomer?.username} - ${formatDateId(selectedTakingRecord.takenDate)}`
            : "Klik edit pada data"}
          description={selectedTakingRecord
            ? `Klik Ok Untuk update data ${selectedCustomer?.username} hari ${formatDateId(selectedTakingRecord.takenDate)}`
            : ""}
          defaultValue={selectedTakingRecord ? selectedTakingRecord.amount : 0}
          disabled={selectedTakingRecord === undefined}
          handleOnConfirm={handleOnUpdateTakingRecord}
          validatorFunction={valueMustBeNonZero}
        />
      }
    </>

  useEffect(() => {
    setCustomers(loaderData.activeCustomer)
  }, [])

  useEffect(() => {
    loadTakingRecords(selectedDate)
  }, [selectedDate])

  const drawerBody = () =>
    <Stack>
      {selectedDate && (
        <Flex>

          <Box minWidth={"400px"}>
            {renderVirtualKeyboard()}
          </Box>

          <Box width={"100%"}>
            <Scroller minHeight="85vh">
              <Stack>
                <Table.Root
                  striped
                  strokeLinecap={'round'}
                  size="lg"
                  variant={'line'}>

                  <Table.Header>
                    <Table.Row>
                      <Table.ColumnHeader>Username</Table.ColumnHeader>
                      <Table.ColumnHeader>Date</Table.ColumnHeader>
                      <Table.ColumnHeader>Record</Table.ColumnHeader>
                    </Table.Row>
                  </Table.Header>

                  <Table.Body>
                    {customers.map((customer) => (
                      <Table.Row key={customer.id}>

                        <Table.Cell>
                          <Flex alignItems={'center'} gap={3}>
                            <Avatar.Root>
                              <Avatar.Fallback name={customer.username} />
                            </Avatar.Root>

                            <Text textStyle={'xl'}>
                              {customer.username}
                            </Text>
                          </Flex>
                        </Table.Cell>

                        <Table.Cell>
                          <Text textStyle={'xl'}>
                            {formatDateId(selectedDate)}
                          </Text>
                        </Table.Cell>

                        <Table.Cell>
                          <DataList.Root orientation={'horizontal'}>

                            {
                              takingRecords.length > 0
                              && takingRecords
                                .filter(pr => pr.userId === customer.id!)
                                .map(record =>
                                  <React.Fragment key={record.id}>
                                    {dataListItemValue(
                                      formatDateId(record.takenDate, "HH:mm"),
                                      `${record.amount} Ampas`,
                                      record.isPaid ? <Badge colorPalette="green">Terbayar</Badge> : undefined
                                    )}
                                    <Button
                                      disabled={record.isPaid}
                                      onClick={() => handleOnEditTakingRecordButtonClick(record)}
                                      maxWidth={"200px"}
                                    >
                                      <AiFillEdit />
                                      Edit
                                    </Button>
                                  </React.Fragment>
                                )
                            }
                            {
                              !takingRecords.map(record => record.userId).includes(customer.id!)
                              && (
                                <Button
                                  maxWidth={"200px"}
                                  onClick={() => {
                                    setSelectedCustomer(customer)
                                    setIsAddingAddNewRecord(true)
                                  }}
                                >
                                  <AiOutlinePlusSquare />
                                  Add Record
                                </Button>
                              )
                            }

                          </DataList.Root>
                        </Table.Cell>

                      </Table.Row>
                    ))}
                  </Table.Body>

                </Table.Root>

              </Stack>
            </Scroller>
          </Box>

        </Flex>
      )}
    </Stack>

  const editDrawer = () =>
    <Drawer.Root
      size={'full'}
      open={open}
      onOpenChange={(e) => setOpen(e.open)}>
      <Portal>
        <Drawer.Backdrop />

        <Drawer.Positioner>

          <Drawer.Content>
            <Drawer.Header>
              <Drawer.Title>{
                selectedDate
                  ?
                  <Flex alignItems={'center'} gap={4}>
                    <AiTwotoneCalendar />
                    <Text textStyle={'lg'}>
                      {formatDateId(selectedDate)}
                    </Text>
                  </Flex>
                  : null
              }
              </Drawer.Title>
            </Drawer.Header>

            <Drawer.Body>
              {drawerBody()}
            </Drawer.Body>

            <Drawer.CloseTrigger
              onClick={handleOnDrawerClosed}
              asChild>
              <CloseButton size="2xl" />
            </Drawer.CloseTrigger>

          </Drawer.Content>

        </Drawer.Positioner>

      </Portal>
    </Drawer.Root>

  return (
    <Stack>
      <Calendar
        isAdmin
        showNavigator
        onCellClicked={handleOnCalendarCellClicked}
      />
      {editDrawer()}
    </Stack>
  )
}
