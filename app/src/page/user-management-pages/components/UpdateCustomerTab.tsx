import { Box, Button, Flex, Heading, Input, Text, Stack, Card, Avatar, DataList, CheckboxCard, IconButton, Dialog, Portal, CloseButton } from '@chakra-ui/react';
import { GoAlertFill, GoTrash } from "react-icons/go";
import Keyboard, { type SimpleKeyboard } from 'react-simple-keyboard';
import Scroller from '@/component/Scroller';
import { dataListItemValue, textKeyboardLayout, textOrNumberKeyboardDisplay } from '@/utility';
import { useEffect, useRef, useState } from 'react';
import type { UserModel } from '@/api-models';
import { userManagementCommand } from '@/commands';
import { format } from 'date-fns';
import { id } from 'date-fns/locale';

export default function UpdateCustomerTab() {

  const [selectedCustomer, setSelectedCustomer] = useState<UserModel | undefined>(undefined)
  const [confirmDeleteDialogOpen, setConfirmDeleteDialogOpen] = useState<boolean>(false)
  const [deleteCandidate, setDeleteCandidate] = useState<UserModel | undefined>(undefined)
  const [customers, setCustomers] = useState<UserModel[]>([])

  const keyboardRef = useRef<SimpleKeyboard>(null)

  const handleOnDeleteCustomer = (customer: UserModel) => {
    setSelectedCustomer(customer)
  }

  const customerCard = (customer: UserModel, withDialog: boolean = false) =>
    <Card.Root>

      <Card.Header>
        <Flex justifyContent={'space-between'}>
          <Flex alignItems={'center'} gap={2}>
            <Avatar.Root>
              <Avatar.Fallback name={customer.username} />
            </Avatar.Root>
            <Text>
              {customer.username}
            </Text>
          </Flex>
          {withDialog && (
            <Flex gap={2}>
              <Button
                onClick={() => handleOnDeleteCustomer(customer)}
              >
                Edit
              </Button>

              {deleteDialogConfirmation(customer)}
            </Flex>

          )}
        </Flex>
      </Card.Header>

      <Card.Body>
        <DataList.Root>
          {dataListItemValue("Pelanggan Sejak:", `${format(customer.createdDate, "PPPP", { locale: id })}`)}
          tambahkan total ambil<br />
          dan total uang terbayarkan etc
        </DataList.Root>
      </Card.Body>

    </Card.Root>

  const deleteDialogConfirmation = (customer: UserModel) =>
    <Dialog.Root>

      <Dialog.Trigger asChild>
        <IconButton colorPalette={'red'}>
          <GoTrash />
        </IconButton>
      </Dialog.Trigger>

      <Portal>
        <Dialog.Backdrop />

        <Dialog.Positioner>
          <Dialog.Content>

            <Dialog.Header>
              <Dialog.Title>
                <Flex alignItems={'center'} gap={3}>
                  <GoAlertFill color='orange' size={'40px'} />
                  Hapus {customer.username} dan data-nya?
                </Flex>
              </Dialog.Title>
            </Dialog.Header>

            <Dialog.Body>
              <Text textStyle={'lg'}>
                Seluruh data pelanggan akan dihapus dari database.
              </Text>
              {customerCard(customer)}
            </Dialog.Body>

            <Dialog.Footer>

              <Dialog.ActionTrigger asChild>
                <Button variant="outline">Batal</Button>
              </Dialog.ActionTrigger>

              <Button colorPalette={'red'}>Hapus</Button>

            </Dialog.Footer>

            <Dialog.CloseTrigger asChild>
              <CloseButton size="sm" />
            </Dialog.CloseTrigger>

          </Dialog.Content>
        </Dialog.Positioner>

      </Portal>

    </Dialog.Root>

  useEffect(() => {
    userManagementCommand.getAllUser()
      .then(customers => setCustomers(customers))
  }, [])

  useEffect(() => {
    if (selectedCustomer && keyboardRef.current)
      keyboardRef.current?.setInput(selectedCustomer.username)
  }, [selectedCustomer])

  return (
    <Flex>
      <Stack>
        <Heading>
          Pilih Pelanggan
        </Heading>

        <Scroller leftNavigation>
          {
            customers.map((customer) => customerCard(customer, true))
          }
        </Scroller>
      </Stack>


      <Stack visibility={selectedCustomer ? 'visible' : 'hidden'}>
        <Heading>
          Nama Pelanggan
        </Heading>

        <Card.Root minWidth={'500px'}>
          <Card.Header>
            <Heading>
              Editing {selectedCustomer?.username}
            </Heading>
          </Card.Header>

          <Card.Body gap={5}>

            <Box>
              <Input
                value={selectedCustomer?.username}
                size={'2xl'}
                placeholder="contoh: sukijan"
                variant="subtle"
              />
            </Box>

            <Box>
              <Heading>
                Status
              </Heading>
              <CheckboxCard.Root
                onCheckedChange={() => selectedCustomer && setSelectedCustomer({
                  ...selectedCustomer,
                  isActive: !selectedCustomer.isActive
                })}
                checked={selectedCustomer?.isActive}>
                <CheckboxCard.HiddenInput />
                <CheckboxCard.Control>

                  <CheckboxCard.Label>
                    {selectedCustomer?.isActive ? 'Aktif' : 'Non-Aktif'}
                  </CheckboxCard.Label>

                  <CheckboxCard.Indicator
                  />
                </CheckboxCard.Control>
              </CheckboxCard.Root>
            </Box>

            <Keyboard
              keyboardRef={(kb) => keyboardRef.current = kb as SimpleKeyboard}
              layout={textKeyboardLayout}
              display={textOrNumberKeyboardDisplay}
              onChange={(username) => selectedCustomer && setSelectedCustomer({
                ...selectedCustomer,
                username
              })}
            />

            <Button>
              Update Customer
            </Button>

          </Card.Body>

        </Card.Root>
      </Stack>

    </Flex>

  )
}
