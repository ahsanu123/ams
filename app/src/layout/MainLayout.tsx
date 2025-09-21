import Clock from "../component/Clock";
import { Outlet, useNavigate } from "react-router";
import amsLogo from "../svg/ams-icon.svg"
import { useMainLayoutStore } from "@/state";
import { SecretRoutes } from "@/routes";
import "./MainLayout.css";

export default function HomePage() {
  const navigate = useNavigate();
  const headerInformation = useMainLayoutStore(state => state.headerInformation)

  const handleAdminLogin = () => {
    navigate(`${SecretRoutes.AdminLoginPage}`)
  }

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
            <sub>Ampas Management System</sub>
          </div>

          <div>
            <Clock />
          </div>
        </div>

        <div className='header-information'>
          <h2>{headerInformation.title}</h2>
          <sub>{headerInformation.description}</sub>
        </div>

      </header>

      <hr />

      <Outlet />

      <footer>
        <sub>
          <button
            onClick={() => handleAdminLogin()}
            className="button-transparent"
          > ©️</button>
          Copyright {new Date().getFullYear()}
        </sub>
      </footer>
    </>
  )

}
