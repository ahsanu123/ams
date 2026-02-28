import { ActionIcon, Flex, Group, Image, Title } from "@mantine/core"
import { MdAdminPanelSettings } from "react-icons/md";
import { IoStatsChart } from "react-icons/io5";
import amsLogo from "../svg/ams-icon.svg";
import { SideBarComponentType, useSidebarStore } from "@/global-stores/right-sidebar-store";

interface BottomInformation {
}

export default function BottomInformation(_props: BottomInformation) {
  const setDisplayedComponentType = useSidebarStore(store => store.setDisplayedComponent);
  return (
    <Flex
      p={"2px 10px"}
      justify={"space-between"}
      align={'center'}
    >
      <Flex gap={15}>
        <Image
          w={50}
          h={40}
          src={amsLogo}
        />
        <Title>AMS</Title>
      </Flex>

      <Group gap={15}>
        <ActionIcon
          size={"60px"}
          variant="filled"
          onClick={() => setDisplayedComponentType(SideBarComponentType.Statistics)}
        >
          <IoStatsChart size={"60px"} />
        </ActionIcon>

        <ActionIcon
          size={"60px"}
          variant="filled"
          onClick={() => setDisplayedComponentType(SideBarComponentType.AdminLogin)}
        >
          <MdAdminPanelSettings size={"60px"} />
        </ActionIcon>
      </Group>
    </Flex>
  )
}
