import TreeMenuComponent from "@/page/admin-page/components/TreeMenu";
import { AppRoutes } from "@/routes";
import { useMainLayoutStore } from "@/state";
import { Outlet, useNavigate } from "react-router";
import Clock from "../component/Clock";
import amsLogo from "../svg/ams-icon.svg";
import "./AdminLayout.css";
import { useAdminPageStore } from "@/page";
import { useEffect } from "react";

export default function AdminLayout() {
  const navigate = useNavigate();
  const headerInformation = useMainLayoutStore(state => state.headerInformation)
  const menuPath = useAdminPageStore(state => state.menuPath)
  const menuPathId = useAdminPageStore(state => state.menuPathId)

  const handleOnBackToCustomerTakingPage = () => {
    navigate(`${AppRoutes.Root}`)
  }

  useEffect(() => {
    if (menuPathId !== undefined) {
      navigate(menuPathId)
    }
  }, [menuPath])

  return (
    <>
      <header>
        <div>
          <div>
            <div
              className="logo-name"
            >
              <img
                height={55}
                width={56}
                src={amsLogo}
              />
              <h1>
                AMS
              </h1>
            </div>
          </div>

          <div>
            <Clock />
          </div>

          <div>
            <sub>🏠 {menuPath.replaceAll('/', '➡️')}</sub>
          </div>
        </div>

        <div className='header-information'>
          <h2>{headerInformation.title}</h2>
          <sub>{headerInformation.description}</sub>
        </div>

      </header>

      <hr />

      <div className="outlet">
        <div style={{ flex: 1 }}>
          <TreeMenuComponent />
        </div>
        <hr />
        <div
          className="button-container"
          style={{ flex: 5 }}>
          <Outlet />
        </div>
      </div>

      <footer>
        <sub>
          <button
            onClick={() => handleOnBackToCustomerTakingPage()}
            className="button-transparent"
          > ©️</button>
          Copyright {new Date().getFullYear()}
        </sub>
      </footer>
    </>
  )

}
