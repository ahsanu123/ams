import { Stack, Text, Heading, Center } from "@chakra-ui/react";

export default function ReportPage() {
  return (
    <Center
      backgroundSize={'contain'}
      minHeight={'500px'}>
      <Stack>

        <Heading size={'6xl'}>
          AMS - Admin
        </Heading>

        <Text>
          use left menu to manage data.
        </Text>

      </Stack>
    </Center>
  )
}
