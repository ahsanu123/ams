import type { TakingRecordModel } from "@/api-models"
import Calendar from "@/component/Calendar"
import Scroller from "@/component/Scroller"
import VirtualKeypad from "@/component/VirtualKeypad"
import { Text, Box, Button, CloseButton, Drawer, Flex, Heading, Portal, Stack, Card, Avatar, DataList } from "@chakra-ui/react"
import { format } from "date-fns"
import { id } from "date-fns/locale"
import { useState } from "react"
import { AiTwotoneCalendar } from "react-icons/ai"

enum ListActionEnum {
  GetPageModel = 'GetPageModel',
  MakePayment = 'MakePayment'
}

interface IFetcherActionResult {
}

interface IGetPageModelClientRequest {
  userId: number,
  date: Date,
  _action: ListActionEnum,
}

export async function clientLoader() {
  return {
  }
}

export async function clientAction() {

}


export default function UpdateTakingRecordPage() {

  const [open, setOpen] = useState(false)
  const [selectedDate, setSelectedDate] = useState<Date | undefined>(undefined)
  const [takingRecord, setTakingRecord] = useState<TakingRecordModel | undefined>(undefined)

  const handleOnCalendarCellClicked = (date: Date | undefined) => {
    setOpen(true)
    setSelectedDate(date)
  }

  const handleOnPickDregs = (value: number) => {
  }
  const valueMustBeNonZero = (value: number) => {
    if (value == 0) return "Tidak Boleh Nol (0)"
    return undefined
  }

  const dataListItemValue = (item: string, value: string) =>
    <>
      <DataList.Item>

        <DataList.ItemLabel justifyContent={'space-between'}>
          <Text textStyle={'lg'} fontWeight={'bold'}>
            {item}
          </Text>
          <Button>Edit</Button>
        </DataList.ItemLabel>

        <DataList.ItemValue>
          <Text textStyle={'lg'} fontWeight={'bold'}>
            {value}
          </Text>
        </DataList.ItemValue>

      </DataList.Item>
      <hr />
    </>

  const drawerBody = () =>
    <Stack>
      <Flex>
        <VirtualKeypad
          title="Klik edit pada data"
          disabled={takingRecord == undefined}
          handleOnConfirm={handleOnPickDregs}
          validatorFunction={valueMustBeNonZero}
        />

        <Box width={100}>
          <Scroller>
            <Stack>

              {Array(2).fill(0).map((item) =>
                <Card.Root>

                  <Card.Header>
                    <Flex gap={2} justifyContent={'left'} alignItems={'center'}>
                      <Avatar.Root>
                        <Avatar.Fallback name="Paijo" />
                      </Avatar.Root>
                      <Heading>
                        User Name
                      </Heading>
                    </Flex>
                  </Card.Header>

                  <Card.Body>
                    <DataList.Root>
                      {dataListItemValue("29 Januri 2025, 14:30", "2 Ampas")}
                      {dataListItemValue("29 Januri 2025, 14:30", "23 Ampas")}
                      {dataListItemValue("29 Januri 2025, 14:30", "12 Ampas")}
                    </DataList.Root>
                  </Card.Body>

                  <Card.Footer>
                  </Card.Footer>

                </Card.Root>
              )}

            </Stack>
          </Scroller>
        </Box>
      </Flex>
    </Stack>

  const editDrawer = () =>
    <Drawer.Root
      size={'xl'}
      open={open}
      onOpenChange={(e) => setOpen(e.open)}>
      <Portal>
        <Drawer.Backdrop />

        <Drawer.Positioner>

          <Drawer.Content>
            <Drawer.Header>
              <Drawer.Title>{
                selectedDate
                  ?
                  <Text>
                    <AiTwotoneCalendar />
                    {format(selectedDate, "PPPP", { locale: id })}
                  </Text>
                  : null
              }
              </Drawer.Title>
            </Drawer.Header>

            <Drawer.Body>
              {drawerBody()}
            </Drawer.Body>

            <Drawer.Footer>
              <Button variant="outline">Cancel</Button>
              <Button>Save</Button>
            </Drawer.Footer>

            <Drawer.CloseTrigger asChild>
              <CloseButton size="sm" />
            </Drawer.CloseTrigger>

          </Drawer.Content>

        </Drawer.Positioner>

      </Portal>
    </Drawer.Root>

  return (
    <Stack>
      <Calendar
        isAdmin
        showNavigator
        onCellClicked={handleOnCalendarCellClicked}
      />
      {editDrawer()}
    </Stack>
  )
}
