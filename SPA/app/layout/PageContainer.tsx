import { Outlet, useNavigate } from "react-router";

export default function PageContainer() {
  const navigate = useNavigate();
  return (

    <>
      <button
        onClick={() => navigate("/")}
      >
        🏡 Back Home
      </button>

      <Outlet />
    </>
  )
}
