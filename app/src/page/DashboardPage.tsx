import { useNavigate } from "react-router"
import './DashboardPage.css'
import Calendar from "../component/Calendar";
import { useMainLayoutStore } from "@/state";
import { mockCommand } from "@/commands";
import type { Route } from "./+types/DashboardPage";
import { useEffect } from "react";

export async function clientLoader() {
  const listUser = await mockCommand.getUsers()
  const productRecord = await mockCommand.getProductRecord()

  return {
    listUser,
    productRecord
  }
}


export default function DashboardPage({
  loaderData
}: Route.ComponentProps) {

  const { productRecord, listUser } = loaderData

  useEffect(() => {
    useMainLayoutStore.setState({
      listUser,
      products: productRecord
    })
  }, [])

  return (
    <>
      <div
        className="dashboard-page"
      >
        <main>
          <Calendar
            showNavigator
            onNextMonthClicked={(date) => undefined}
            onPrevMonthClicked={(date) => undefined}
          />
        </main>

      </div>
    </>
  )
}
