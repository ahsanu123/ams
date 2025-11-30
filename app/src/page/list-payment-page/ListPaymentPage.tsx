import { customerMoneyCommand, paymentHistoryCommand, userManagementCommand } from "@/commands";
import Scroller from "@/component/Scroller";
import { EMPTY_HEADER_INFORMATION } from "@/constants";
import { useMainLayoutStore } from "@/state";
import { dataListItemValue, formatAsRupiah, formatDateId } from "@/utility";
import { Badge, Box, Card, DataList, Flex, Heading, Image, Stack, Table } from "@chakra-ui/react";
import { useEffect } from "react";
import DatePicker from "react-datepicker";
import type { Route } from "./+types/ListPaymentPage";
import { useListPaymentPageState } from "./list-payment-page-state";
import './ListPaymentPage.css';

export async function clientLoader() {
  const activeCustomer = await userManagementCommand.getAllActiveUser()
  return {
    activeCustomer
  }
}

export default function ListPaymentPage({
  loaderData
}: Route.ComponentProps) {

  const { activeCustomer } = loaderData

  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)

  const listCustomer = useListPaymentPageState(state => state.listCustomer)
  const setListCustomer = useListPaymentPageState(state => state.setListCustomer)

  const selectedCustomer = useListPaymentPageState(state => state.selectedCustomer)
  const setSelectedCustomer = useListPaymentPageState(state => state.setSelectedCustomer)

  const selectedDate = useListPaymentPageState(state => state.selectedDate)
  const setSelectedDate = useListPaymentPageState(state => state.setSelectedDate)

  const paymentRecords = useListPaymentPageState(state => state.paymentRecords)
  const setPaymentRecords = useListPaymentPageState(state => state.setPaymentRecords)

  const moneyHistories = useListPaymentPageState(state => state.moneyHistories)
  const setMoneyHistories = useListPaymentPageState(state => state.setMoneyHistories)

  const catatanPembayaran = () =>
    <Stack maxWidth={'1000px'} >
      {moneyHistory()}
      {paymentHistory()}
    </Stack>

  const moneyHistory = () =>
    <Scroller
      title={`Catatan Uang ${selectedCustomer?.username}`}
      maxHeight="350px"
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
            moneyHistories.map(money =>
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

  const paymentHistory = () =>

    <Scroller
      title={`Catatan Pembayaran ${selectedCustomer?.username}`}
      maxHeight="350px"
    >
      <Table.Root stickyHeader>
        <Table.Header>
          <Table.Row>
            <Table.ColumnHeader>
              Tanggal
            </Table.ColumnHeader>

            <Table.ColumnHeader>
              Tagihan
            </Table.ColumnHeader>

            <Table.ColumnHeader>
              Penambahan uang
            </Table.ColumnHeader>

            <Table.ColumnHeader>
              Uang Awal
            </Table.ColumnHeader>

            <Table.ColumnHeader>
              Uang Akhir
            </Table.ColumnHeader>

          </Table.Row>
        </Table.Header>
        <Table.Body>
          {
            paymentRecords.map((payment) =>
              <Table.Row colorPalette={'green'}>
                <Table.Cell>
                  {formatDateId(payment.date)}
                </Table.Cell>

                <Table.Cell>
                  {formatAsRupiah(payment.billAmount)}
                </Table.Cell>

                <Table.Cell>
                  {formatAsRupiah(payment.addedMoney)}
                </Table.Cell>

                <Table.Cell>
                  {formatAsRupiah(payment.initialMoney)}
                </Table.Cell>

                <Table.Cell>
                  {formatAsRupiah(payment.endMoney)}
                </Table.Cell>
              </Table.Row>
            )
          }
        </Table.Body>

      </Table.Root>
    </Scroller>

  useEffect(() => {

    if (selectedCustomer && selectedDate) {
      paymentHistoryCommand.getPaymentRecordByUserIdAndMonth(selectedCustomer.id, selectedDate)
        .then((records) => setPaymentRecords(records))

      customerMoneyCommand.getAllUserMoneyHistory(selectedCustomer.id)
        .then((moneyHistories) => setMoneyHistories(moneyHistories))
    }

  }, [selectedCustomer, selectedDate])

  useEffect(() => {
    setListCustomer(activeCustomer)
    setHeaderInformation({
      title: 'List Payment',
      description: 'Place To See All Taking Record'
    })

    return () => setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }, [])

  return (
    <Box className="list-payment-page">

      <Flex className="flex-left-and-right">
        <Stack className="name-and-date-picker">
          <Heading>
            Pilih Nama
          </Heading>
          <select
            onChange={(event) => setSelectedCustomer(listCustomer.find(pr => pr.id === Number(event.currentTarget.value)))}
          >
            <option value="">Nama</option>
            {
              listCustomer.map((customer, index) => (
                <option
                  key={index}
                  value={customer.id}
                >
                  {customer.username}
                </option>
              ))
            }
          </select>

          {selectedCustomer && (
            <Card.Root>
              <Card.Header>
                <Heading>
                  Data {selectedCustomer.username}
                </Heading>
              </Card.Header>

              <Card.Body>
                <DataList.Root>
                  {dataListItemValue("Status", selectedCustomer.isActive ? "Aktif" : "Tidak Aktif")}
                  {dataListItemValue("Uang Saat Ini", formatAsRupiah(selectedCustomer.money))}
                  {dataListItemValue("Pelanggan Sejak", formatDateId(selectedCustomer.createdDate))}
                </DataList.Root>
              </Card.Body>
            </Card.Root>
          )}

          <Heading>
            Pilih Bulan dan Tahun
          </Heading>
          <DatePicker
            inline
            placeholderText="Bulan Dan Tahun"
            dateFormat="MMMM yyyy "
            selected={selectedDate}
            onChange={(date) => date && setSelectedDate(date)}
            showMonthYearPicker
          />
        </Stack>

        {paymentRecords.length > 0 && catatanPembayaran()}
        {
          selectedDate &&
          selectedCustomer &&
          paymentRecords.length <= 0 &&
          <Stack alignItems={'center'}>
            <Heading>Tidak Ada Data</Heading>
            <Image
              src="/ams-hero-2.png"
            />
          </Stack>
        }

      </Flex>

    </Box>
  )
}
