import VirtualKeypad from "@/components/VirtualKeypad";
import { useSidebarStore } from "@/global-stores/right-sidebar-store";
import { AdminRoutes } from "@/routes";
import { calculatePassword } from "@/utilities/password-calculator";
import { useEffect } from "react";
import { useNavigate } from "react-router";

export default function AdminLoginSidebar() {

  const setSidebarTitle = useSidebarStore(store => store.setTitle)
  const navigate = useNavigate()

  const handleOnConfirm = (password: number) => {
    const calculatedPassword = calculatePassword();

    if (password === calculatedPassword) {
      navigate(`${AdminRoutes.AdminRoot}${AdminRoutes.GreetPage}`)
    }
  }

  useEffect(() => {
    setSidebarTitle("Masukan Password")
  }, [])

  return (
    <>
      <VirtualKeypad
        handleOnConfirm={handleOnConfirm}
      />
    </>
  )
}
