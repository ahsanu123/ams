import Calendar from "../component/Calendar";
import { useMainLayoutStore } from "@/state";
import { getCommand, mockCommand } from "@/commands";
import type { Route } from "./+types/DashboardPage";
import { useEffect } from "react";
import './DashboardPage.css'

export async function clientLoader() {
  const command = getCommand();

  const listUser = await command.getUsers()
  const productRecord = await command.getProductRecord()
  const productPrice = await command.getProductPrice()

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
    productRecord,
    listUser,
    productPrice,
  } = loaderData

  const command = getCommand();

  const selectedMonth = useMainLayoutStore(state => state.selectedMonth)
  const setAllProductOfThisMonth = useMainLayoutStore(state => state.setAllProductOfThisMonth)

  // TODO: Think better way to do this
  const getAllProduct = async () => {
    const allProductOfThisMonth = await command.getAllProductOfThisMonth(selectedMonth)
    setAllProductOfThisMonth(allProductOfThisMonth)
  }

  useEffect(() => {
    useMainLayoutStore.setState({
      products: productRecord,
      listUser,
      productPrice,
    })
    getAllProduct()
  }, [])

  return (
    <>
      <div
        className="dashboard-page"
      >
        <main>
          {/* <Calendar /> */}
        </main>

      </div>
    </>
  )
}
