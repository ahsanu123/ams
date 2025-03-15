import { useNavigate } from "react-router"
import type { Route } from "./+types/DashboardPage";
import './DashboardPage.css'
import Calendar from "../component/Calendar";
import { generateMockProduct } from "@/mock";
import { useProductStore } from "@/state";

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

useProductStore.setState({
  productsCell: [
    {
      type: "HeaderLabel"
    }
  ]
});

export default function DashboardPage({
  loaderData
}: Route.ComponentProps) {


  const productRecord = useProductStore((state) => state.products)

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
