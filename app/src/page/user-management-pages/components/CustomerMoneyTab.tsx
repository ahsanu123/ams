import { Box, Button, Flex, Heading, Text, Stack, Card, Avatar, Table, NumberInput, Badge } from '@chakra-ui/react';
import Keyboard, { type SimpleKeyboard } from 'react-simple-keyboard';
import 'react-simple-keyboard/build/css/index.css';
import Scroller from '@/component/Scroller';
import { formatAsRupiah, formatDateId, numberLayout, textOrNumberKeyboardDisplay, toaster } from '@/utility';
import { useEffect, useRef, useState } from 'react';
import type { UserModel } from '@/api-models';
import { customerMoneyCommand, useCustomerMoneyCommand, useUserManagementCommand } from '@/commands';
import { AiFillDollarCircle } from 'react-icons/ai';
import { useQuery } from '@tanstack/react-query';

const MINIMUM_ADD_MONEY = 10000

export default function CustomerMoneyTab() {

  const keyboardRef = useRef<SimpleKeyboard>(null)
  const [selectedCustomer, setSelectedCustomer] = useState<UserModel | undefined>(undefined)

  const { getAllUser } = useUserManagementCommand()
  const { getAllUserMoneyHistory } = useCustomerMoneyCommand()

  const { data: customers, refetch: refetchAllCustomer } = useQuery(getAllUser)
  const { data: moneyHistory, refetch: refetchMoneyHistory } = useQuery(getAllUserMoneyHistory(selectedCustomer?.id));

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
        title: `Berhasil Menambahakan ${formatAsRupiah(addMoneyAmount)} ke ${selectedCustomer.username}`,
        type: 'success'
      });
      return;
    }
  }

  useEffect(() => {
    refetchAllCustomer()
    refetchMoneyHistory()

    setAddMoneyAmount(0)
    keyboardRef.current?.setInput('0')
  }, [selectedCustomer])

  const listCustomer = () =>
    <Stack maxWidth={'400px'}>
      <Scroller
        title='Uang Pelanggan'
        minHeight='70vh'
        leftNavigation
      >
        {
          customers?.map((customer) =>
            <Card.Root>
              <Card.Body>
                <Flex justifyContent={'space-between'}>
                  <Stack>
                    <Flex alignItems={'center'} gap={5}>
                      <Avatar.Root>
                        <Avatar.Fallback name={customer.username} />
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
        moneyHistory &&
        moneyHistory.length > 0 &&
        (
          <Box>
            <Scroller
              maxHeight='300px'
              title='Catatan transaksi'
            >
              <Table.Root stickyHeader>
                <Table.Header>
                  <Table.Row>
                    <Table.ColumnHeader>
                      Tanggal
                    </Table.ColumnHeader>
                    <Table.ColumnHeader>
                      Jumlah uang
                    </Table.ColumnHeader>
                    <Table.ColumnHeader>
                      Keterangan
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
                          {
                            (money.description.toLocaleLowerCase().includes('paying') ||
                              money.description.toLocaleLowerCase().includes('Dept'))
                              ? <Badge colorPalette={'green'}>Pay</Badge>
                              : <Badge colorPalette={'blue'}>Add</Badge>
                          }
                          {' '}
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

      {moneyHistory && moneyHistory.length <= 0 && selectedCustomer && (
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
          onKeyPress={() => undefined}
        />
        <Button
          height={'80px'}
          fontSize={'2xl'}
          onClick={() => handleOnAddMoney()}
        >
          Add Money
        </Button>
      </Stack>
    </Stack >


  return (
    <Flex gap={"150px"}>
      {listCustomer()}
      {moneyEditor()}
    </Flex>
  )
}
