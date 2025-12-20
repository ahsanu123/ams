import { Box, Button, Flex, Heading, Input, Text, Stack, Card, Avatar, DataList, CheckboxCard, IconButton, Dialog, Portal, CloseButton, Spinner } from '@chakra-ui/react';
import { GoAlertFill, GoTrash } from "react-icons/go";
import Keyboard, { type SimpleKeyboard } from 'react-simple-keyboard';
import Scroller from '@/component/Scroller';
import { dataListItemValue, textKeyboardLayout, textOrNumberKeyboardDisplay, toaster } from '@/utility';
import { useEffect, useRef, useState } from 'react';
import type { UserModel } from '@/api-models';
import { useCustomerMoneyCommand, useUserManagementCommand } from '@/commands';
import { format } from 'date-fns';
import { id } from 'date-fns/locale';
import { useQuery } from '@tanstack/react-query';

export default function UpdateCustomerTab() {

  const [selectedCustomer, setSelectedCustomer] = useState<UserModel | undefined>(undefined)
  const [deleteCandidate, setDeleteCandidate] = useState<UserModel | undefined>(undefined)
  const [dialogOpen, setDialogOpen] = useState<boolean>(false)

  const { deleteUser } = useCustomerMoneyCommand()
  const { upsertUser, getAllUser } = useUserManagementCommand()

  const { data: customers, refetch: refetchCustomer } = useQuery(getAllUser)

  const keyboardRef = useRef<SimpleKeyboard>(null)

  const handleOnEditCustomer = (customer: UserModel) => {
    setSelectedCustomer(customer)
  }

  const handleOnCofirmDeleteCustomer = async () => {
    if (!deleteCandidate) return;

    const res = await deleteUser.mutateAsync({ userId: deleteCandidate.id })

    if (res > 0) {
      toaster.create({
        title: `Succes To Delete ${deleteCandidate.username}, ${res}`,
        type: 'success'
      })
      setSelectedCustomer(undefined)
      setDeleteCandidate(undefined)
      setDialogOpen(false)
    }
    else {
      toaster.create({
        title: `Error To Delete ${deleteCandidate.username}`,
        type: 'error'
      })
      setSelectedCustomer(undefined)
      setDeleteCandidate(undefined)
      setDialogOpen(false)
    }
    await refetchCustomer()
  }

  const handleOnUpdateCustomer = async () => {
    if (selectedCustomer === undefined) return
    if (selectedCustomer.username === '') {
      toaster.create({
        title: 'Username Cannot Be Empty',
        type: 'info'
      })
      return;
    }

    const id = await upsertUser.mutateAsync({ user: selectedCustomer })

    if (id === 0)
      toaster.create({
        title: `Fail To Update ${selectedCustomer.username}`,
        type: 'error'
      })
    else
      toaster.create({
        title: `Success to update ${selectedCustomer.username}`,
        type: 'success'
      })

    setSelectedCustomer(undefined)
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
              {customer.isActive ? '' : " (Tidak Aktif)"}
            </Text>
          </Flex>
          {withDialog && (
            <Flex gap={2}>
              <Button
                onClick={() => handleOnEditCustomer(customer)}
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
          {dataListItemValue(
            "Pelanggan Sejak:",
            `${format(customer.createdDate, "PPPP", { locale: id })}`
          )}
        </DataList.Root>
      </Card.Body>

    </Card.Root>

  const deleteDialogConfirmation = (customer: UserModel) =>
    <Dialog.Root
      open={dialogOpen}
      onOpenChange={(e) => setDialogOpen(e.open)}>

      <Dialog.Trigger asChild>
        <IconButton
          onClick={() => setDeleteCandidate(customer)}
          colorPalette={'red'}>
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
                <Button
                  variant="outline">
                  Batal
                </Button>
              </Dialog.ActionTrigger>

              <Button
                colorPalette={'red'}
                onClick={() => handleOnCofirmDeleteCustomer()}
              >
                Hapus
              </Button>

            </Dialog.Footer>

            <Dialog.CloseTrigger asChild>
              <CloseButton size="sm" />
            </Dialog.CloseTrigger>

          </Dialog.Content>
        </Dialog.Positioner>

      </Portal>

    </Dialog.Root>

  useEffect(() => {
    if (selectedCustomer && keyboardRef.current)
      keyboardRef.current?.setInput(selectedCustomer.username)
  }, [selectedCustomer])

  if (!customers) return (
    <Stack>
      <Spinner />
      <Text>User Empty</Text>
    </Stack>
  )

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

        <Card.Root width={'700px'}>
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

            <Button
              height={'80px'}
              fontSize={'2xl'}
              onClick={() => handleOnUpdateCustomer()}
            >
              Update Customer
            </Button>

          </Card.Body>

        </Card.Root>
      </Stack>

    </Flex>

  )
}
