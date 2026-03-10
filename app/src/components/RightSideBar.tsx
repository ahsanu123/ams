import { useCurrentPath } from "@/utilities/useCurrentPath";
import { Group, Stack, Title, Text } from "@mantine/core";
import { ReactNode } from "react";
import VirtualKeypad from "./VirtualKeypad";
import { SideBarComponentType, useSidebarStore } from "@/global-stores/right-sidebar-store";
import MainPageCustomerPicker from "@/sidebars-component/MainPageCustomerPicker";
import AdminLoginSidebar from "@/sidebars-component/AdminLoginSidebar";
import AdminMenuSidebar from "@/sidebars-component/AdminMenuSidebar";

interface ReactNodeWithId {
  id: string,
  component: () => ReactNode
}

const routeToSideBarMap = new Map<SideBarComponentType, ReactNodeWithId>([
  [SideBarComponentType.PaymentPage, {
    id: "payment",
    component: () => <VirtualKeypad handleOnConfirm={() => undefined} />
  }],
  [SideBarComponentType.AdminLogin, {
    id: "admin-login",
    component: () => <AdminLoginSidebar />,
  }],
  [SideBarComponentType.Statistics, {
    id: "statistic",
    component: () => <Text>this some text</Text>
  }],
  [SideBarComponentType.MainPageCustomerPicker, {
    id: "main-page-customer-picker",
    component: () => <MainPageCustomerPicker />
  }],
  [SideBarComponentType.AdminMenu, {
    id: "admin-menu",
    component: () => <AdminMenuSidebar />
  }],
]);

export default function RightSideBar() {

  const { path: _ } = useCurrentPath()
  const displayedComponent = useSidebarStore(store => store.displayedComponent);
  const title = useSidebarStore(store => store.title);

  const header = () => (
    <Group>
      <Title>{title}</Title>
    </Group>
  )

  return (
    <Stack
      align="stretch"
      p={"5px 3px"}
    >
      {header()}
      {routeToSideBarMap.get(displayedComponent)?.component()}
    </Stack>
  )
}
