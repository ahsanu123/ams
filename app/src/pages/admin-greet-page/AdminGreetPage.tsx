import { SideBarComponentType, useSidebarStore } from "@/global-stores/right-sidebar-store"
import { Center, Image, Stack } from "@mantine/core";
import { useEffect } from "react"
import amsLogo from "../../svg/ams-icon.svg";

export default function AdminGreetPage() {
  const setSidebarTitle = useSidebarStore(store => store.setTitle)
  const setSidebarComponent = useSidebarStore(store => store.setDisplayedComponent);

  useEffect(() => {
    setSidebarTitle("Welcome!!")
    setSidebarComponent(SideBarComponentType.AdminMenu)
  }, [])
  return (
    <Center
      h={"100%"}
    >
      <Stack align="center">
        <Image
          w={250}
          h={200}
          src={amsLogo}
        />
      </Stack>
    </Center>
  )
}
