import { getProductCommand, getUserCommand } from "@/commands";
import './MainAdminPage.css';
import type { Route } from "./+types/MainAdminPage";
import { useMainAdminPageState } from "@/state";

export async function clientLoader() {
  const productCommand = getProductCommand();
  const userCommand = getUserCommand();
  return {
  }
}

export default function MainAdminPage({
  loaderData
}: Route.ComponentProps) {

  const { } = loaderData

  const menus = useMainAdminPageState(state => state.menus)

  return (
    <div>

      <div>

      </div>

      <div>
      </div>

    </div>
  )
}
