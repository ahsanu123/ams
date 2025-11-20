import { Box, Button, Flex, Heading, Input, Text, Stack, Card, Avatar, DataList, Table, Spacer, NumberInput } from '@chakra-ui/react';
import Keyboard, { type SimpleKeyboard } from 'react-simple-keyboard';
import 'react-simple-keyboard/build/css/index.css';
import Scroller from '@/component/Scroller';
import { dataListItemValue, formatAsRupiah, formatDateId, numberLayout, textKeyboardLayout, textOrNumberKeyboardDisplay, toaster } from '@/utility';
import { useEffect, useRef, useState } from 'react';
import type { MoneyHistoryModel, UserModel } from '@/api-models';
import { customerMoneyCommand, userManagementCommand } from '@/commands';
import { AiFillDollarCircle } from 'react-icons/ai';

const MINIMUM_ADD_MONEY = 10000

export default function CustomerMoneyTab() {

  const keyboardRef = useRef<SimpleKeyboard>(null)
  // FIXME: move this state to its own state file 
  const [selectedCustomer, setSelectedCustomer] = useState<UserModel | undefined>(undefined)
  const [customers, setCustomers] = useState<UserModel[]>([])
  const [moneyHistory, setMoneyHistory] = useState<MoneyHistoryModel[]>([])
  const [addMoneyAmount, setAddMoneyAmount] = useState<number>(0)

  const handleOnAddMoney = () => {
    if (addMoneyAmount < MINIMUM_ADD_MONEY) {
      toaster.create({
        title: `Minimum ${formatAsRupiah(MINIMUM_ADD_MONEY)}`,
        type: 'error'
      });
      return;
    }

    if (selectedCustomer && addMoneyAmount >= MINIMUM_ADD_MONEY) {
      customerMoneyCommand.addMoney(selectedCustomer.id, addMoneyAmount)
        .then((customer) => setSelectedCustomer(customer))

      toaster.create({
        title: `Berhasil Menambahakan ${formatAsRupiah(MINIMUM_ADD_MONEY)} ke ${selectedCustomer.username}`,
        type: 'success'
      });
      return;
    }
  }

  const getAllUserMoney = () => {

    if (selectedCustomer)
      customerMoneyCommand.getAllUserMoneyHistory(selectedCustomer.id)
        .then((moneys) => setMoneyHistory(moneys))
  }

  const loadAllUser = () => {
    userManagementCommand.getAllActiveUser()
      .then((users) => setCustomers(users))
  }

  useEffect(() => {
    setAddMoneyAmount(0)
    loadAllUser()
    getAllUserMoney()
    keyboardRef.current?.setInput('0')
  }, [selectedCustomer])

  useEffect(() => {
    loadAllUser()
  }, [])

  const listCustomer = () =>
    <Stack maxWidth={'400px'}>
      <Scroller
        title='Uang Pelanggan'
        minHeight='70vh'
        leftNavigation
      >
        {
          customers.map((customer) =>
            <Card.Root>
              <Card.Body>
                <Flex justifyContent={'space-between'}>
                  <Stack>
                    <Flex alignItems={'center'} gap={5}>
                      <Avatar.Root>
                        <Avatar.Fallback name='user name' />
                      </Avatar.Root>
                      <Text>
                        {customer.username}
                      </Text>
                    </Flex>

                    <Text>
                      Pelanggan Sejak {formatDateId(customer.createdDate)}
                    </Text>
                    <Text>
                      {customer.money < 0 ? 'Hutang' : 'Uang'}
                      {' '}
                      {formatAsRupiah(customer.money)}
                    </Text>
                  </Stack>
                  <Button
                    onClick={() => setSelectedCustomer(customer)}
                  >
                    Choose
                  </Button>
                </Flex>
              </Card.Body>

            </Card.Root>
          )
        }
      </Scroller>
    </Stack>

  const moneyEditor = () =>
    <Stack>
      {
        selectedCustomer &&
        moneyHistory.length > 0 &&
        (
          <Box>
            <Scroller
              maxHeight='300px'
              title='List Money History'
            >
              <Table.Root stickyHeader>
                <Table.Header>
                  <Table.Row>
                    <Table.ColumnHeader>
                      Date
                    </Table.ColumnHeader>
                    <Table.ColumnHeader>
                      Amount
                    </Table.ColumnHeader>
                    <Table.ColumnHeader>
                      Description
                    </Table.ColumnHeader>
                  </Table.Row>
                </Table.Header>
                <Table.Body>
                  {
                    moneyHistory.map(money =>
                      <Table.Row>
                        <Table.Cell>
                          {formatDateId(money.date)}
                        </Table.Cell>

                        <Table.Cell>
                          {formatAsRupiah(money.moneyAmount)}
                        </Table.Cell>

                        <Table.Cell>
                          {money.description}
                        </Table.Cell>
                      </Table.Row>
                    )
                  }
                </Table.Body>

              </Table.Root>
            </Scroller>
          </Box>
        )}

      {moneyHistory.length <= 0 && selectedCustomer && (
        <>
          <Heading size={'4xl'}>
            <Flex gap={5} alignItems={'center'}>
              <AiFillDollarCircle />
              {selectedCustomer?.username} tidak punya uang
            </Flex>
          </Heading>
          <hr />
        </>
      )}

      <Stack visibility={selectedCustomer ? '' : 'hidden'}>
        <Heading>
          Tambahkan {selectedCustomer?.username} uang
        </Heading>
        <NumberInput.Root
          value={addMoneyAmount.toString()}
          formatOptions={{
            style: "currency",
            currency: "IDR",
          }}
        >
          <NumberInput.Control />
          <NumberInput.Input fontSize={'2xl'} />
        </NumberInput.Root>
        <Keyboard
          keyboardRef={(kb) => keyboardRef.current = kb as SimpleKeyboard}
          layout={numberLayout}
          display={textOrNumberKeyboardDisplay}
          onChange={(value) => setAddMoneyAmount(Number(value))}
        />
        <Button
          onClick={() => handleOnAddMoney()}
        >
          Add Money
        </Button>
      </Stack>
    </Stack >


  return (
    <Flex gap={"200px"}>
      {listCustomer()}
      {moneyEditor()}
    </Flex>
  )
}
