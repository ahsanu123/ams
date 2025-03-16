import { useNavigate } from "react-router"
import type { Route } from "./+types/DashboardPage";
import './DashboardPage.css'
import Calendar from "../component/Calendar";
import { useMainLayoutStore } from "@/state";
import { isInsideTauri, viteEnv, whatEnvironmentIs } from "@/commands";
import { useEffect } from "react";

export async function clientLoader() {
  // return await whoami()
  // const users = await LocalAccountService.getLocalAccountListUser({
  //   client: defaultClient
  // })
  //
  // return {
  //   users: users.data as string[]
  // }
}

useMainLayoutStore.setState({
  user: {
    id: 1,
    username: "Paijo",
    isActive: true,
    money: 1000000,
    bill: 0
  },
  products: [
    {
      id: 1,
      userId: 1,
      paid: true,
      productionDate: new Date(2025, 2, 2),
      takenDate: new Date(2025, 2, 2),
      price: 11000,
      amount: 1,
      description: "taking 1"
    },
    {
      id: 1,
      userId: 1,
      paid: false,
      productionDate: new Date(2025, 2, 10),
      takenDate: new Date(2025, 2, 10),
      price: 11000,
      amount: 2,
      description: "taking 1"
    },
    {
      id: 1,
      userId: 1,
      paid: false,
      productionDate: new Date(2025, 2, 11),
      takenDate: new Date(2025, 2, 11),
      price: 11000,
      amount: 3,
      description: "taking 1"
    },
    {
      id: 1,
      userId: 1,
      paid: false,
      productionDate: new Date(2025, 2, 12),
      takenDate: new Date(2025, 2, 12),
      price: 11000,
      amount: 4,
      description: "taking 1"
    },
    {
      id: 1,
      userId: 1,
      paid: false,
      productionDate: new Date(2025, 2, 13),
      takenDate: new Date(2025, 2, 13),
      price: 11000,
      amount: 6,
      description: "taking 1"
    },
    {
      id: 1,
      userId: 1,
      paid: false,
      productionDate: new Date(2025, 3, 25),
      takenDate: new Date(2025, 3, 25),
      price: 11000,
      amount: 2,
      description: "taking 1"
    },
    {
      id: 1,
      userId: 1,
      paid: false,
      productionDate: new Date(2025, 3, 1),
      takenDate: new Date(2025, 3, 1),
      price: 11000,
      amount: 2,
      description: "taking 1"
    }
  ]
});

export default function DashboardPage({
  loaderData
}: Route.ComponentProps) {


  const productRecord = useMainLayoutStore((state) => state.products)

  const navigate = useNavigate();
  const refreshPage = () => {
    window.location.reload()
  }

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
