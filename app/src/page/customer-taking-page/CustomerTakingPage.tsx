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

export default function DashboardPage({
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
    <div className="user-container">
      {
        listUser.map((user, index) => (
          <button
            key={index}
            onClick={() => handleOnClickUser(user)}
          >
            {user.username}
          </button>
        ))
      }
    </div>
  )

  const showVirtualKeypad = () => (
    <div>
      <VirtualKeypad
        title="Ambil Berapa?"
        inputType='number'
        handleOnConfirm={handleOnPickDregs}
        validatorFunction={valueMustBeNonZero}
      />
    </div>
  )

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
    <>
      <div
        className="customer-taking-page"
      >
        <main>
          {
            customer ? showVirtualKeypad() : showUserSelector()
          }

          <div>
            <Calendar />
          </div>
        </main>
      </div>
    </>
  )
}
