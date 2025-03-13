import Clock from "../component/Clock"
import Calendar from "../component/Calendar"
import ListUser from "../component/ListUser"
import { useState } from "react"
import UserBillInformation from "../component/UserBillInformation"
import type { Route } from "./+types/AMSAdminPage"
import { utilToDate } from "../utility/convert-heyapi-date"
import PaidForDuration from "../component/PaidForDuration"

/*
 * Basic AMS admin flow 
 *
 * admin able to look every user ampas summary each week or month, 
 * there a display for total ampas and total bill for each selected user.
 * there is print button to print bill.
 *
 * */

export async function clientLoader() {
  const users = await LocalAccountService.getLocalAccountListUser({
    client: defaultClient
  })

  return {
    users: users.data as string[]
  }
}

export default function AMSAdminPage({
  loaderData
}: Route.ComponentProps) {

  const {
    users
  } = loaderData

  const [calendarDate, setCalendarDate] = useState<Date>(new Date());
  const [selectedUser, setSelectedUser] = useState<string>()
  const [monthlyInformation, setMontlyInformation] = useState<AmpasModel[]>([]);

  const getMontlyUserInformation = async (username: string, date?: Date) => {

    await LocalAccountService.getLocalAccountLogout({ client: defaultClient })
    await LocalAccountService.postLocalAccountLoginWithoutPassword({
      client: defaultClient,
      query: { username }
    })
    setSelectedUser(username)
    const monthlyInformation = await AmpasPricingService.postAmpasPricingMonthlyInformation({
      client: defaultClient,
      body: date ?? calendarDate
    })

    setMontlyInformation(monthlyInformation.data as AmpasModel[])
  }

  const renderTakenByUserForThisDay = (date: string) => (
    // make it simpler
    monthlyInformation?.filter((item) => {
      const takenTime = utilToDate(item.takenTime!)
      // console.log(takenTime.getDate())
      return takenTime.getDate() === parseInt(date)
    })
      .map((item) => item.amount)
      .reduce((a, b) => ((a ?? 0) + (b ?? 0)), 0)
  )

  const isPaidForThisDate = (date: string) => {
    const ispaid = monthlyInformation?.find((item) =>
      utilToDate(item.takenTime!).getDate() === parseInt(date)
    )
    return ispaid?.paid
  }

  const adminGridElement = (date: string) => {
    const includeDash = date.includes("-")
    return (
      <div
        className={`component ${includeDash ? "stripe" : ""}`}
      >
        {!includeDash
          ? (
            <>
              <p>
                {selectedUser
                  ? (
                    <>
                      {isPaidForThisDate(date)
                        ? `✔️ ${renderTakenByUserForThisDay(date)} `
                        : (`Ambil ${renderTakenByUserForThisDay(date)}`)}
                    </>
                  )
                  : (
                    "Pilih User"
                  )
                }
              </p>

            </>
          )
          : (
            <p>
              🚧
            </p>
          )}
        <sub>
          <b>
            {date}
          </b>
        </sub>
      </div>
    )
  }

  return (
    <>
      <h2>AMS Admin Page</h2>
      <Clock />

      <hr />
      <h4>Monthly Worth</h4>


      <Calendar
        title={selectedUser}
        gridComponent={adminGridElement}
        onNextMonthClicked={(date) => {
          setCalendarDate(date)
          getMontlyUserInformation(selectedUser ?? "", date)
        }}
        onPrevMonthClicked={(date) => {
          setCalendarDate(date)
          getMontlyUserInformation(selectedUser ?? "", date)
        }}
      />

      <hr />
      <b>Daftar Pengguna</b>
      <ListUser
        users={users}
        handleOnUserSelected={(username) => getMontlyUserInformation(username)}
      />

      {selectedUser && (
        <>
          <hr />
          <PaidForDuration
            username={selectedUser}
            onSubmit={() => getMontlyUserInformation(selectedUser)}
          />
          <hr />
        </>
      )}
      <UserBillInformation />
    </>
  )
}
