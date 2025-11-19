import { dataListItemValue, textKeyboardLayout, textOrNumberKeyboardDisplay, toaster } from '@/utility';
import { Avatar, Text, Button, Card, Flex, Heading, Input, Stack, Toaster, DataList } from '@chakra-ui/react';
import { AiFillContacts } from 'react-icons/ai';
import Keyboard, { type SimpleKeyboard } from 'react-simple-keyboard';
import { useCreateNewCustomerTabState } from './create-new-customer-tab-state';
import { useEffect, useRef, useState } from 'react';
import { userManagementCommand } from '@/commands';
import Scroller from '@/component/Scroller';
import type { UserModel } from '@/api-models';
import { format } from 'date-fns';
import { id } from 'date-fns/locale';

enum KeyboardChar {
  Enter = "{enter}"
}
export default function CreateNewCustomerTab() {

  const newUserName = useCreateNewCustomerTabState(state => state.newUserName)
  const setNewUserName = useCreateNewCustomerTabState(state => state.setNewUserName)
  const [customers, setCustomers] = useState<UserModel[]>([])

  const keyboardRef = useRef<SimpleKeyboard>(null)

  const handleOnNewCustomerKeyPress = (button: string) => {
    if (button === KeyboardChar.Enter) {
    }
  }

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
          {dataListItemValue("Pelanggan Sejak:", `${format(customer.createdDate, "PPPP", { locale: id })}`)}
          tambahkan total ambil<br />
          dan total uang terbayarkan etc
        </DataList.Root>
      </Card.Body>

    </Card.Root>

  const handleOnCreateNewCustomer = () => {
    if (newUserName === '') return;

    userManagementCommand.createNewUser(newUserName)
      .then(id => {
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
      })

  }

  useEffect(() => {
    userManagementCommand.getAllUser()
      .then(customers => setCustomers(customers))
  }, [])

  return (
    <Stack className='create-new-user-tab-content'>
      <Heading size={'2xl'}>
        Masukan Nama
      </Heading>

      <Flex gap={2}>
        <Stack className='keyboard-container'>
          <Input
            value={newUserName}
            size={'2xl'}
            placeholder="contoh: sukijan"
            variant="subtle"
          />
          <Keyboard
            onKeyPress={handleOnNewCustomerKeyPress}
            keyboardRef={kb => keyboardRef.current = kb as SimpleKeyboard}
            onChange={(value, _) => setNewUserName(value)}
            layout={textKeyboardLayout}
            display={textOrNumberKeyboardDisplay}
          />
          <Button
            disabled={newUserName === ''}
            onClick={handleOnCreateNewCustomer}
          >
            <AiFillContacts />
            Buat Pelanggan Baru
          </Button>
        </Stack>


        <Scroller>
          {
            customers.map((customer) => customerCard(customer))
          }
        </Scroller>

      </Flex>
    </Stack>
  )
}
