import Calendar from "../component/Calendar";
import { useMainLayoutStore } from "@/state";
import { getProductCommand, getUserCommand } from "@/commands";
import type { Route } from "./+types/DashboardPage";
import { useEffect } from "react";
import './DashboardPage.css'
import VirtualKeypad from "@/component/VirtualKeypad";
import { EMPTY_HEADER_INFORMATION } from "@/constants";
import type { User } from "@/model";

export async function clientLoader() {
  const productCommand = getProductCommand();
  const userCommand = getUserCommand();

  const listUser = await userCommand.getUsers()
  const productRecord = await productCommand.getProductRecord()
  const productPrice = await productCommand.getProductPrice()

  return {
    listUser,
    productRecord,
    productPrice,
  }
}

export default function DashboardPage({
  loaderData
}: Route.ComponentProps) {

  const {
    listUser,
  } = loaderData

  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)

  const user = useMainLayoutStore(state => state.user)
  const setUser = useMainLayoutStore(state => state.setUser)

  const setLastSelectedUser = useMainLayoutStore(state => state.setLastSelectedUser)

  useEffect(() => {
    setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }, [])

  const handleOnPickDregs = (value: number) => {
    console.log(`${user?.username} is pick ${value} dregs`)

    setUser(undefined)
    setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }

  const valueMustBeNonZero = (value: number): string | undefined => {
    if (value == 0) return "Tidak Boleh Nol (0)"
    return undefined
  }

  const handleOnClickUser = (user: User) => {
    setLastSelectedUser(user)
    setUser(user)
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

  return (
    <>
      <div
        className="dashboard-page"
      >
        <main>
          {
            user ? showVirtualKeypad() : showUserSelector()
          }

          <div>
            <Calendar />
          </div>
        </main>
      </div>
    </>
  )
}
