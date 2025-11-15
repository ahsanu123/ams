import { Image, Box, Button, Flex, Heading, Input, Text, Stack, Tabs, Card, Avatar, DataList, Field, CheckboxCard } from '@chakra-ui/react';
import { AiFillContacts } from 'react-icons/ai';
import Keyboard from 'react-simple-keyboard';
import 'react-simple-keyboard/build/css/index.css';
import { useUserManagementPageState } from './user-management-page-state';
import './UserManagementPage.css';
import Scroller from '@/component/Scroller';
import { formatAsRupiah } from '@/utility';

enum UserManagementTabs {
  CreateNewCustomer = "Create New Customer",
  UpdateCustomer = "Update Customer",
  CustomerMoneyManagement = "Customer Money"
}

export default function UserManagementPage() {
  const newUserName = useUserManagementPageState(state => state.newUserName)
  const setNewUserName = useUserManagementPageState(state => state.setNewUserName)

  const onKeyboardNewCustomerChanged = (value: string) => setNewUserName(value)

  const dataListItemValue = (item: string, value: string) =>
    <DataList.Item>

      <DataList.ItemLabel>
        <Text textStyle={'lg'} fontWeight={'bold'}>
          {item}
        </Text>
      </DataList.ItemLabel>

      <DataList.ItemValue>
        <Text textStyle={'lg'} fontWeight={'bold'}>
          {value}
        </Text>
      </DataList.ItemValue>

    </DataList.Item>

  const createNewUserTabContent = () =>
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
            onChange={onKeyboardNewCustomerChanged}
          />
          <Button>
            <AiFillContacts />
            Buat Pelanggan Baru
          </Button>
        </Stack>

      </Flex>
    </Stack>

  const updateCustomerTabContent = () =>
    <Flex>
      <Stack>
        <Card.Root>
          <Card.Header>
            <Heading>
              Editing User Name
            </Heading>
          </Card.Header>

          <Card.Body gap={5}>

            <Box>
              <Heading>
                Nama Pelanggan
              </Heading>
              <Input
                size={'2xl'}
                placeholder="contoh: sukijan"
                variant="subtle"
              />
            </Box>

            <Box>
              <Heading>
                Status
              </Heading>
              <CheckboxCard.Root>
                <CheckboxCard.HiddenInput />
                <CheckboxCard.Control>
                  <CheckboxCard.Label> Aktif / Non-Aktif</CheckboxCard.Label>
                  <CheckboxCard.Indicator />
                </CheckboxCard.Control>
              </CheckboxCard.Root>
            </Box>

            <Keyboard />

          </Card.Body>

        </Card.Root>
      </Stack>

      <Stack>
        <Heading>
          Pilih Pelanggan
        </Heading>

        <Scroller>
          {
            Array(10).fill(0).map((item) =>
              <Card.Root>

                <Card.Header>
                  <Flex justifyContent={'space-between'}>
                    <Flex alignItems={'center'} gap={2}>
                      <Avatar.Root>
                        <Avatar.Fallback name='user name' />
                      </Avatar.Root>
                      <Text>
                        User Name
                      </Text>
                    </Flex>
                    <Button>
                      Edit
                    </Button>
                  </Flex>
                </Card.Header>

                <Card.Body>
                  <DataList.Root>
                    {dataListItemValue("Pelanggan Sejak", "20 juli 1854")}
                  </DataList.Root>
                </Card.Body>

              </Card.Root>
            )
          }
        </Scroller>
      </Stack>

    </Flex>

  const customerMoneyManagementTabContent = () =>
    <Stack>
      <Heading>

      </Heading>

      <Flex>
        <Scroller
          title='Uang Pelanggan'
          leftNavigation
        >
          {
            Array(10).fill(0).map(item =>
              <Card.Root>
                <Card.Header>
                  <Flex justifyContent={'space-between'}>
                    <Flex alignItems={'center'} gap={5}>
                      <Avatar.Root>
                        <Avatar.Fallback name='user name' />
                      </Avatar.Root>
                      <Text>
                        User Name
                      </Text>
                    </Flex>
                    <Button>
                      Choose
                    </Button>
                  </Flex>
                </Card.Header>

                <Card.Body>
                  <DataList.Root>
                    {dataListItemValue("Uang", `${formatAsRupiah(1900000)}`)}
                    {dataListItemValue("Hutang", `${formatAsRupiah(1900000)}`)}
                  </DataList.Root>
                </Card.Body>

              </Card.Root>
            )
          }
        </Scroller>

        <Scroller
          title='List Money History'
          leftNavigation
        >
          {
            Array(10).fill(0).map(item =>
              <Card.Root>
                <Card.Header>
                  <Flex justifyContent={'space-between'}>
                    <Flex alignItems={'center'} gap={5}>
                      <Avatar.Root>
                        <Avatar.Fallback name='user name' />
                      </Avatar.Root>
                      <Text>
                        User Name
                      </Text>
                    </Flex>
                    <Button>
                      Choose
                    </Button>
                  </Flex>
                </Card.Header>

                <Card.Body>
                  <DataList.Root>
                    {dataListItemValue("Uang", `${formatAsRupiah(1900000)}`)}
                    {dataListItemValue("Hutang", `${formatAsRupiah(1900000)}`)}
                  </DataList.Root>
                </Card.Body>

              </Card.Root>
            )
          }
        </Scroller>

        <Box>
          <Heading>
            Tambahkan Uang Ke User name
          </Heading>
          <Input
            type='number'
            size={'2xl'}
            placeholder="contoh: 90000"
            variant="subtle"
          />
          <Keyboard />
          <Button>
            Add Money
          </Button>
        </Box>

      </Flex>

    </Stack>

  return (
    <Box>
      <Tabs.Root defaultValue={UserManagementTabs.CreateNewCustomer}>
        <Tabs.List>

          <Tabs.Trigger value={UserManagementTabs.CreateNewCustomer}>
            {UserManagementTabs.CreateNewCustomer}
          </Tabs.Trigger>

          <Tabs.Trigger value={UserManagementTabs.UpdateCustomer}>
            {UserManagementTabs.UpdateCustomer}
          </Tabs.Trigger>

          <Tabs.Trigger value={UserManagementTabs.CustomerMoneyManagement}>
            {UserManagementTabs.CustomerMoneyManagement}
          </Tabs.Trigger>

        </Tabs.List>

        <Tabs.Content
          value={UserManagementTabs.CreateNewCustomer}>
          {createNewUserTabContent()}
        </Tabs.Content>

        <Tabs.Content
          value={UserManagementTabs.UpdateCustomer}>
          {updateCustomerTabContent()}
        </Tabs.Content>

        <Tabs.Content
          value={UserManagementTabs.CustomerMoneyManagement}>
          {customerMoneyManagementTabContent()}
        </Tabs.Content>

      </Tabs.Root>
    </Box>
  )
}
