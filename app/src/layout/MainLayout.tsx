import Clock from "../component/Clock";
import { Outlet, useNavigate } from "react-router";
import amsLogo from "../svg/ams-icon.svg"
import { useMainLayoutStore } from "@/state";
import "./MainLayout.css";
import { formatAsRupiah } from "@/utility/format-as-rupiah";
import { useEffect } from "react";

export default function HomePage() {
  const navigate = useNavigate();

  const user = useMainLayoutStore(state => state.user)
  const onNextPage = useMainLayoutStore(state => state.nextPage)
  const onPrevPage = useMainLayoutStore(state => state.prevPage)
  const currentPage = useMainLayoutStore(state => state.currentPage)

  useEffect(() => {
    navigate(currentPage)
  }, [currentPage])

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

        <div
          className="user-profile"
        >
          <h5>
            🏃 {user && user.username}
          </h5>
          <sub>
            💶 Uang Simpanan : <b>{user && formatAsRupiah(user.money)}</b>
          </sub>
          <sub>
            💷 Total Tagihan Seluruhnya : <b>{user && formatAsRupiah(user.money)}</b> (TODO)
          </sub>
          <sub>
            📍 Total Ambil Tahun {new Date().getFullYear()} : <b>200</b> (TODO)
          </sub>
        </div>

      </header>


      <hr />

      <Outlet />

      <footer>
        <sub>
          ©️ Copyright {new Date().getFullYear()}
        </sub>

        <div>
          <button
            className="button-transparent"
            onClick={() => onNextPage()}
          >
            ▶️
          </button>
          <button
            className="button-transparent"
            onClick={() => onPrevPage()}
          >
            ◀️
          </button>
        </div>
      </footer>
    </>
  )

}
