import { dataListItemValue, textKeyboardLayout, textOrNumberKeyboardDisplay, toaster } from '@/utility';
import { Avatar, Text, Button, Card, Flex, Heading, Input, Stack, DataList } from '@chakra-ui/react';
import { AiFillContacts } from 'react-icons/ai';
import Keyboard, { type SimpleKeyboard } from 'react-simple-keyboard';
import { useCreateNewCustomerTabState } from './create-new-customer-tab-state';
import { useRef } from 'react';
import { userManagementCommand, useUserManagementCommand } from '@/commands';
import Scroller from '@/component/Scroller';
import type { UserModel } from '@/api-models';
import { format } from 'date-fns';
import { id } from 'date-fns/locale';
import { useQuery } from '@tanstack/react-query';

export default function CreateNewCustomerTab() {

  const newUserName = useCreateNewCustomerTabState(state => state.newUserName)
  const setNewUserName = useCreateNewCustomerTabState(state => state.setNewUserName)

  const { getAllUser, createNewUser } = useUserManagementCommand()
  const { data: customers } = useQuery(getAllUser)

  const keyboardRef = useRef<SimpleKeyboard>(null)

  const customerCard = (customer: UserModel) =>
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

  const handleOnCreateNewCustomer = async () => {
    if (newUserName === '') {
      toaster.create({
        title: 'username is empty!!',
        type: 'error'
      })
      return;
    };

    const id = await createNewUser.mutateAsync({ username: newUserName })
    if (id === 0)
      toaster.create({
        title: `Fail to create new customer named ${newUserName}, username Already Taken`,
        type: 'error'
      })
    else
      toaster.create({
        title: `Success to create new customer named ${newUserName}`,
        type: 'success'
      })
    setNewUserName('')
    keyboardRef.current?.setInput('')
  }

  return (
    <Stack className='create-new-user-tab-content'>
      <Heading size={'2xl'}>
        Masukan Nama
      </Heading>

      <Flex gap={2}>
        <Stack className='keyboard-container'>
          <Input
            defaultValue={""}
            value={newUserName}
            size={'2xl'}
            placeholder="contoh: sukijan"
            variant="subtle"
          />
          <Keyboard
            keyboardRef={kb => keyboardRef.current = kb as SimpleKeyboard}
            onChange={(value, _) => setNewUserName(value)}
            layout={textKeyboardLayout}
            display={textOrNumberKeyboardDisplay}
          />
          <Button
            height={'80px'}
            fontSize={'2xl'}
            disabled={newUserName === ''}
            onClick={handleOnCreateNewCustomer}
          >
            <AiFillContacts />
            Buat Pelanggan Baru
          </Button>
        </Stack>


        <Scroller>
          {
            customers && customers.map((customer) => customerCard(customer))
          }
        </Scroller>

      </Flex>
    </Stack>
  )
}
