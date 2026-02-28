import { ActionIcon, Center, Flex, Group, Image, Title } from "@mantine/core"
import { ReactNode } from "react"
import { MdAdminPanelSettings } from "react-icons/md";
import { IoStatsChart } from "react-icons/io5";
import amsLogo from "../svg/ams-icon.svg";

interface BottomInformation {
  child?: ReactNode
}

export default function BottomInformation(props: BottomInformation) {
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
        >
          <IoStatsChart size={"60px"} />
        </ActionIcon>

        <ActionIcon
          size={"60px"}
          variant="filled"
        >
          <MdAdminPanelSettings size={"60px"} />
        </ActionIcon>
      </Group>
    </Flex>
  )
}
