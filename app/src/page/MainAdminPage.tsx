import { getProductCommand, getUserCommand } from "@/commands";
import type { Route } from "./+types/MainAdminPage";
import { useMainAdminPageState } from "@/state";
import type { FlatMenuTree } from "@/model";
import './MainAdminPage.css';

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

  const activeMenu = useMainAdminPageState(state => state.activeMenu)
  const setActiveMenu = useMainAdminPageState(state => state.setActiveMenu)

  const selectedMenu = useMainAdminPageState(state => state.selectedMenu)
  const setSelectedMenu = useMainAdminPageState(state => state.setSelectedMenu)
  const setActiveMenuBackToRoot = useMainAdminPageState(state => state.setActiveMenuBackToRoot)

  const handleOnMenuClicked = (menu: FlatMenuTree) => {

    setActiveMenu(menu.groupName)

    if (menu.component)
      setSelectedMenu(menu)
  }

  return (
    <div className="main-admin-page">

      <div>
        <button
          onClick={() => setActiveMenuBackToRoot()}
        >
          Root
        </button>
        {
          activeMenu.map((menu) => (
            <button
              onClick={() => handleOnMenuClicked(menu)}
            >
              {menu.title}
            </button>
          ))
        }
      </div>

      <div>
        {selectedMenu?.component}
      </div>

    </div>
  )
}
