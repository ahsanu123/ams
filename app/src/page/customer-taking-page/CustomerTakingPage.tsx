import type { UserModel } from "@/api-models";
import { userManagementCommand, useTakingRecordCommand, useUserManagementCommand } from "@/commands";
import Calendar from "@/component/Calendar";
import VirtualKeypad from "@/component/VirtualKeypad";
import { EMPTY_HEADER_INFORMATION } from "@/constants";
import { useMainLayoutStore } from "@/state";
import { Box, Button, Flex, Grid, GridItem, Stack } from "@chakra-ui/react";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import type { Route } from "./+types/CustomerTakingPage";
import './CustomerTakingPage.css';

export async function clientLoader() {
  const listUser = await userManagementCommand.getAllActiveUser();
  return { listUser }
}

export default function CustomerTakingPage({
  loaderData
}: Route.ComponentProps) {

  const { listUser } = loaderData

  const customer = useMainLayoutStore(state => state.customer)
  const setCustomer = useMainLayoutStore(state => state.setCustomer)
  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)

  const lastSelectedUser = useMainLayoutStore(state => state.lastSelectedUser)
  const setLastSelectedUser = useMainLayoutStore(state => state.setLastSelectedUser)

  const { getAllActiveUser } = useUserManagementCommand()
  const { data: activeUsers } = useQuery(getAllActiveUser())

  const {
    getTakingRecordByUserIdAndMonth,
    addNewtakingRecord
  } = useTakingRecordCommand();

  const { data: userTakingRecords } = useQuery(getTakingRecordByUserIdAndMonth(
    lastSelectedUser?.id ?? activeUsers?.[0].id,
    new Date()
  ));

  const handleOnPickDregs = (amount: number) => {
    if (!customer) return;

    addNewtakingRecord.mutate({
      user_id: customer.id,
      amount
    })

    setCustomer(undefined)
    setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }

  const handleOnCustomerIdChange = (customerId: number) => {
    const customer = activeUsers?.find(pr => pr.id === customerId)
    if (customer) setLastSelectedUser(customer)
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
          <GridItem key={index}>
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

  useEffect(() => {
    setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }, [])


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
          onCustomerIdChange={handleOnCustomerIdChange}
        />
      </Flex>
    </Box >
  )
}
