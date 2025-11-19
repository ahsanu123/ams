import React from "react"
import Calendar from "@/component/Calendar"
import Scroller from "@/component/Scroller"
import VirtualKeypad from "@/component/VirtualKeypad"
import { Text, Box, Button, CloseButton, Drawer, Flex, Portal, Stack, Avatar, DataList, Table, Badge, Steps } from "@chakra-ui/react"
import { useEffect, useState } from "react"
import { AiFillEdit, AiOutlinePlusSquare, AiTwotoneCalendar } from "react-icons/ai"
import { useUpdateTakingPageState } from "./update-taking-record-page-state"
import { takingRecordCommand, userManagementCommand } from "@/commands"
import type { Route } from "./+types/UpdateTakingRecordPage"
import { dataListItemValue, formatDateId, toaster } from "@/utility"
import type { TakingRecordModel, UserModel } from "@/api-models"
import { GoTrash } from "react-icons/go"

interface IStep {
  title: string,
  description: string
}

enum EditStep {
  PickUser = 0,
  InsertAmount = 1,
  SaveChanges = 2
}

export async function clientLoader() {
  const activeCustomer = await userManagementCommand.getAllActiveUser()
  return {
    activeCustomer
  }
}

export async function clientAction() { }


export default function UpdateTakingRecordPage({
  loaderData
}: Route.ComponentProps) {


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
  const [editStep, setEditStep] = useState(0)

  const editOrAddStep: IStep[] = [
    {
      title: "Pilih Pelanggan",
      description: "Klik Edit atau Add pada Table di sebelah kanan.",
    },
    {
      title: "Masukan Jumlah",
      description: "Masukan Jumlah Ampas",
    },
  ]

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

    setEditStep(EditStep.InsertAmount)
    setSelectedCustomer(customer)
    setSelectedTakingRecord(record)
  }

  const handleOnDeleteTakingRecord = (record: TakingRecordModel) => {
    takingRecordCommand.deleteTakingRecordById(record.id)
      .then(_ => {
        toaster.create({
          title: `Data Deleted`,
          type: 'success'
        })
        loadTakingRecords(selectedDate)
      }
      )
  }

  const handleOnUpdateTakingRecord = (amount: number) => {
    if (selectedTakingRecord)
      takingRecordCommand.upsertTakingRecord({
        ...selectedTakingRecord,
        amount
      }).then(_ => {
        toaster.create({
          title: `Success Updating, ${amount}`,
          type: 'success'
        })
        setEditStep(EditStep.PickUser)
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
      .then(() => {
        toaster.create({
          title: `${selectedCustomer.username} Pick ${amount} Ampas`,
          type: 'success'
        })
        setEditStep(EditStep.PickUser)
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

  const onAddRecordButtonClicked = (customer: UserModel) => {
    setEditStep(EditStep.InsertAmount)
    setSelectedCustomer(customer)
    setIsAddingAddNewRecord(true)
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

          <Stack minWidth={"400px"}>
            {renderVirtualKeyboard()}

            <Steps.Root
              step={editStep}
              onStepChange={(e) => setEditStep(e.step)}
              size='sm'
              defaultStep={1}
              count={editOrAddStep.length}>
              <Steps.List>
                {editOrAddStep.map((step, index) => (
                  <Steps.Item
                    key={index}
                    index={index}
                    title={step.title}>
                    <Steps.Indicator />
                    <Steps.Title>{step.title}</Steps.Title>
                    <Steps.Separator />
                  </Steps.Item>
                ))}
              </Steps.List>

              {editOrAddStep.map((step, index) => (
                <Steps.Content key={index} index={index}>
                  {step.description}
                </Steps.Content>
              ))}

              <Steps.CompletedContent>All steps are complete!</Steps.CompletedContent>

            </Steps.Root>
          </Stack>

          <Box width={"100%"}>
            <Scroller minHeight="85vh">
              <Stack>
                <Table.Root
                  striped
                  stickyHeader
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

                                    <Flex gap={5}>
                                      <Button
                                        disabled={record.isPaid}
                                        onClick={() => handleOnEditTakingRecordButtonClick(record)}
                                        maxWidth={"200px"}
                                      >
                                        <AiFillEdit />
                                        Edit
                                      </Button>

                                      <Button
                                        disabled={record.isPaid}
                                        onClick={() => handleOnDeleteTakingRecord(record)}
                                        colorPalette={'red'}>
                                        <GoTrash />
                                        Delete
                                      </Button>

                                    </Flex>
                                  </React.Fragment>
                                )
                            }
                            {
                              !takingRecords.map(record => record.userId).includes(customer.id!)
                              && (
                                <Button
                                  maxWidth={"200px"}
                                  onClick={() => onAddRecordButtonClicked(customer)}
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
        hideCustomerAvatar
        onCellClicked={handleOnCalendarCellClicked}
      />
      {editDrawer()}
    </Stack>
  )
}
