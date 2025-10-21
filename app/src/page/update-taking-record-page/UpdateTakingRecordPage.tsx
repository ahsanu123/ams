import Calendar from "@/component/Calendar"
import Scroller from "@/component/Scroller"
import VirtualKeypad from "@/component/VirtualKeypad"
import { Text, Box, Button, CloseButton, Drawer, Flex, Heading, Portal, Stack } from "@chakra-ui/react"
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

  const drawerBody = () =>
    <Stack>
      <Flex>
        <VirtualKeypad
          title="Pilih User"
          inputType='number'
          handleOnConfirm={handleOnPickDregs}
          validatorFunction={valueMustBeNonZero}
        />

        <Box width={100}>
          <Scroller>
            <Text>
              lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).
              Where does it come from?
              Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of "de Finibus Bonorum et Malorum" (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, "Lorem ipsum dolor sit amet..", comes from a line in section 1.10.32.
              The standard chunk of Lorem Ipsum used since the 1500s is reproduced below for those interested. Sections 1.10.32 and 1.10.33 from "de Finibus Bonorum et Malorum" by Cicero are also reproduced in their exact original form, accompanied by English versions from the 1914 translation by H. Rackham.
              Where can I get some?
              There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.
            </Text>
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
