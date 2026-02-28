import { useCurrentPath } from "@/utilities/useCurrentPath";
import { Group, Stack, Title, Text } from "@mantine/core";
import { ReactNode } from "react";
import VirtualKeypad from "./VirtualKeypad";
import { SideBarComponentType, useSidebarStore } from "@/global-stores/right-sidebar-store";
import MainPageCustomerPicker from "@/pages/main-pages/components/MainPageCustomerPicker";

interface ReactNodeWithTitle {
  id: string,
  component: () => ReactNode
}

const routeToSideBarMap = new Map<SideBarComponentType, ReactNodeWithTitle>([
  [SideBarComponentType.PaymentPage, {
    id: "payment",
    component: () => <VirtualKeypad handleOnConfirm={() => undefined} />
  }],
  [SideBarComponentType.AdminLogin, {
    id: "Enter Password",
    component: () => <VirtualKeypad handleOnConfirm={() => undefined} />
  }],
  [SideBarComponentType.Statistics, {
    id: "Statistik",
    component: () => <Text>this some text</Text>
  }],
  [SideBarComponentType.MainPageCustomerPicker, {
    id: "Pilih Nama",
    component: () => <MainPageCustomerPicker />
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
