import Clock from "../component/Clock";
import { Outlet } from "react-router";
import amsLogo from "../svg/ams-icon.svg"
import "./MainLayout.css";

export default function HomePage() {
  return (
    <>
      <header>

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
          <button
            className="button-transparent"
            onClick={() => console.log("on bolt clicked!!")}
          >
            ⚡
          </button>
        </div>

      </header>
      <hr />

      <Outlet />
    </>
  )

}
