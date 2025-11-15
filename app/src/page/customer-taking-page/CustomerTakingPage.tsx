import type { TakingRecordModel, UserModel } from "@/api-models";
import { takingRecordCommand, userManagementCommand } from "@/commands";
import Calendar from "@/component/Calendar";
import VirtualKeypad from "@/component/VirtualKeypad";
import { EMPTY_HEADER_INFORMATION } from "@/constants";
import { useMainLayoutStore } from "@/state";
import { fromFormData, toFormData } from "@/utility";
import { useEffect } from "react";
import { useFetcher } from "react-router";
import type { Route } from "./+types/CustomerTakingPage";
import { Box, Button, Flex, Grid, GridItem, Stack } from "@chakra-ui/react";
import './CustomerTakingPage.css';

interface IAddTakingRecordClientRequest {
  userId: number,
  amount: number
}

export async function clientLoader() {
  const listUser = await userManagementCommand.getAllActiveUser();
  return { listUser }
}

export async function clientAction({ request }: Route.ClientActionArgs) {
  const data = await fromFormData<IAddTakingRecordClientRequest>(request)
  console.log(data)
  await takingRecordCommand.addNewTakingRecord(data.userId, data.amount)

  const takingRecords = await takingRecordCommand.getTakingRecordByUserIdAndMonth(data.userId, new Date())

  return takingRecords
}

export default function CustomerTakingPage({
  loaderData
}: Route.ComponentProps) {

  const {
    listUser,
  } = loaderData

  const fetcher = useFetcher()

  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)

  const customer = useMainLayoutStore(state => state.customer)
  const setCustomer = useMainLayoutStore(state => state.setCustomer)

  const setUserTakingRecords = useMainLayoutStore(state => state.setUserTakingRecords)
  const setLastSelectedUser = useMainLayoutStore(state => state.setLastSelectedUser)

  const userTakingRecords = useMainLayoutStore(state => state.userTakingRecords);

  const lastSelectedUser = useMainLayoutStore(state => state.lastSelectedUser)

  const handleOnPickDregs = (amount: number) => {
    const serializedData = toFormData({
      userId: customer?.id,
      amount
    })

    fetcher.submit(serializedData, {
      method: 'post'
    })

    setCustomer(undefined)
    setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }

  const valueMustBeNonZero = (value: number): string | undefined => {
    if (value == 0) return "Tidak Boleh Nol (0)"
    return undefined
  }

  const handleOnClickUser = (user: UserModel) => {
    setLastSelectedUser(user)
    setCustomer(user)
    setHeaderInformation({
      title: user.username,
      description: 'welcome!'
    })
  }

  const showUserSelector = () => (
    <Grid className="user-container">
      {
        listUser.map((user, index) => (
          <GridItem>
            <Button
              key={index}
              onClick={() => handleOnClickUser(user)}
              colorPalette={'teal'}
            >
              {user.username}
            </Button>
          </GridItem>
        ))
      }
    </Grid>
  )

  const showVirtualKeypad = () => (
    <Stack>
      <VirtualKeypad
        title="Ambil Berapa?"
        inputType='number'
        handleOnConfirm={handleOnPickDregs}
        validatorFunction={valueMustBeNonZero}
      />
    </Stack>
  )

  const onCalendarUserIdChange = (userId: number) => {
    takingRecordCommand.getTakingRecordByUserIdAndMonth(
      userId,
      new Date()
    ).then((value) => setUserTakingRecords(value))
  }

  useEffect(() => {
    setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }, [])

  useEffect(() => {
    const fetcherResult = fetcher.data
    if (fetcherResult !== undefined) {
      setUserTakingRecords(fetcherResult as Array<TakingRecordModel>)
    }
  }, [fetcher.data])


  return (
    <Box
      className="customer-taking-page"
    >
      <Flex justifyContent={'space-evenly'} className="keyboard-and-calendar">
        {
          customer ? showVirtualKeypad() : showUserSelector()
        }

        <Calendar
          customerMode
          user={lastSelectedUser}
          takingRecords={userTakingRecords}
          onCustomerIdChange={onCalendarUserIdChange}
        />
      </Flex>
    </Box >
  )
}
