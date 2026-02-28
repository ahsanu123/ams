import { useSidebarStore } from "@/global-stores/right-sidebar-store";
import { Group, Stack, Title, ScrollArea } from "@mantine/core";

export default function RightSideBar() {

  const { title } = useSidebarStore(store => store);

  const header = () => (
    <Group>
      <Title>{title}</Title>
    </Group>
  )

  return (
    <Stack
      align="stretch"
    >
      {header()}
      <ScrollArea h={'70vh'}>
      </ScrollArea>
    </Stack>
  )
}
