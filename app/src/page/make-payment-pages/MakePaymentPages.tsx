import type { MakePaymentPageModel, TakingRecordModel, TakingRecordWithPrice, UserModel } from "@/api-models"
import { makePaymentCommand, userManagementCommand } from "@/commands"
import Scroller from "@/component/Scroller"
import { EMPTY_HEADER_INFORMATION } from "@/constants"
import { useMainLayoutStore } from "@/state"
import { formatAsRupiah, formatDateId, fromFormData, toFormData } from "@/utility"
import { Badge, Box, Button, Card, DataList, Flex, Heading, Stack, Table, Tabs, Text } from "@chakra-ui/react"
import { format } from "date-fns"
import { id } from "date-fns/locale"
import React, { useEffect, useState } from "react"
import DatePicker from "react-datepicker"
import "react-datepicker/dist/react-datepicker.css"
import { useFetcher } from "react-router"
import type { Route } from "./+types/MakePaymentPages"
import { useMakePaymentPageState } from "./make-payment-page-state"
import './MakePaymentPages.css'
import Calendar from "@/component/Calendar"
import { AiFillCalendar, AiFillDollarCircle, AiFillGolden, AiFillNotification, AiFillSliders } from "react-icons/ai"

enum ListActionEnum {
  GetPageModel = 'GetPageModel',
  MakePayment = 'MakePayment'
}

interface IFetcherActionResult {
  pageModel: MakePaymentPageModel,
  customerData: UserModel
}

interface IGetPageModelClientRequest {
  userId: number,
  date: Date,
  _action: ListActionEnum,
}

export async function clientLoader() {
  const activeCustomer = await userManagementCommand.getAllActiveUser()
  return {
    activeCustomer
  }
}

export async function clientAction({ request }: Route.ClientActionArgs): Promise<IFetcherActionResult> {
  const parsedRequest = await fromFormData<IGetPageModelClientRequest>(request)

  if (parsedRequest._action === ListActionEnum.MakePayment) {
    const pageModel = await makePaymentCommand.makePayment(parsedRequest.userId, parsedRequest.date)
    const customerData = await userManagementCommand.getById(parsedRequest.userId)

    return {
      pageModel,
      customerData
    }
  }

  // ListActionEnum.GetPageModel
  else {
    const pageModel = await makePaymentCommand.getPageModel(parsedRequest.userId, parsedRequest.date)
    const customerData = await userManagementCommand.getById(parsedRequest.userId)

    return {
      pageModel,
      customerData
    }
  }

}

enum ListTabEnum {
  CustomerTakingRecordDetail = 'Daftar Ambil',
  CustomerDetailInformation = 'Informasi Pelanggan',
  CustomerTakingCalendar = 'Kalender'
}

export default function MakePaymentPage({
  loaderData
}: Route.ComponentProps) {

  const { activeCustomer } = loaderData

  const fetcher = useFetcher<IFetcherActionResult>()

  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)

  const listCustomer = useMakePaymentPageState(state => state.listCustomer)
  const setListCustomer = useMakePaymentPageState(state => state.setListCustomer)

  const selectedCustomer = useMakePaymentPageState(state => state.selectedCustomer)
  const setSelectedCustomer = useMakePaymentPageState(state => state.setSelectedCustomer)

  const selectedDate = useMakePaymentPageState(state => state.selectedDate)
  const setSelectedDate = useMakePaymentPageState(state => state.setSelectedDate)

  const showDetailTaking = useMakePaymentPageState(state => state.showDetailTaking)
  const setShowDetailTaking = useMakePaymentPageState(state => state.setShowDetailTaking)

  const pageModel = useMakePaymentPageState(state => state.pageModel)
  const setPageModel = useMakePaymentPageState(state => state.setPageModel)

  const [dateOpen, setDateOpen] = useState(false)

  const handleOnDatePickerChange = (date: Date | null) => {
    if (date) {
      const utcTime = Date.UTC(date.getFullYear(), date.getMonth(), 1)
      setSelectedDate(new Date(utcTime))
      setDateOpen(false)
    }
  }

  const handleOnPayIndividualItem = (takingRecord: TakingRecordModel) => {
  }

  const handleOnPayButtonClicked = () => {
    if (selectedCustomer === undefined) return;

    const serializedData = toFormData({
      userId: selectedCustomer.id,
      date: selectedDate,
      _action: ListActionEnum.MakePayment
    })

    fetcher.submit(serializedData, {
      method: 'post',
    })
  }

  const isPayButtonDisabled = () =>
    selectedDate === undefined
    || (pageModel && pageModel.takingRecords.length <= 0)
    || (pageModel && pageModel.takingRecords.every(pr => pr.takingRecord.isPaid === true))

  const detailedCard = (record: TakingRecordWithPrice) => (
    <Card.Root className={`detailed-card ${record.takingRecord.isPaid ? 'paid' : ''}`} >
      <Card.Header>
        <Flex className="card-header">
          <Heading>
            {selectedCustomer?.username}
          </Heading>
          {
            record.takingRecord.isPaid
              ? <Badge size={'lg'} colorPalette="green">Terbayar</Badge>
              : <Badge size={'lg'} colorPalette="red">Belum Terbayar</Badge>
          }
        </Flex>
      </Card.Header>
      <Card.Body>
        <Flex justifyContent={'space-between'}>
          <Box>
            <Text>{formatDateId(record.takingRecord.takenDate)}</Text>
            <Text>{formatDateId(record.takingRecord.takenDate, "p")}</Text>
            <Text>{formatAsRupiah(record.price.price)}</Text>
          </Box>

          <Box justifyItems={'center'}>
            <Heading size={'4xl'}>{record.takingRecord.amount}</Heading>
            <Text textStyle={'xl'}>Ampas</Text>
          </Box>
        </Flex>
        {
          !record.takingRecord.isPaid && (
            <Button
              onClick={() => handleOnPayIndividualItem(record.takingRecord)}
            >
              <AiFillDollarCircle />
              Bayar Hari Ini
            </Button>
          )
        }
      </Card.Body>

    </Card.Root>
  )

  const userDetailComponent = () => {
    const customerData = pageModel?.customers.find(pr => pr.id === selectedCustomer?.id)
    const items = [
      { id: 1, name: "Laptop", category: "Electronics", price: 999.99 },
      { id: 2, name: "Coffee Maker", category: "Home Appliances", price: 49.99 },
      { id: 3, name: "Desk Chair", category: "Furniture", price: 150.0 },
      { id: 4, name: "Smartphone", category: "Electronics", price: 799.99 },
      { id: 5, name: "Headphones", category: "Accessories", price: 199.99 },
    ]
    return (
      <>
        {customerData !== undefined && (
          <Stack>
            <DataList.Root>
              {dataListItemValue('Nama', `${customerData.username}`)}
              {dataListItemValue(
                `${customerData.money >= 0 ? 'Memiliki Uang ' : 'Kekurangan Uang '} Sebesar: `,
                `${formatAsRupiah(customerData.money)}`
              )}
            </DataList.Root>

            <Table.Root size="sm">
              <Table.Header>
                <Table.Row>
                  <Table.ColumnHeader>Product</Table.ColumnHeader>
                  <Table.ColumnHeader>Category</Table.ColumnHeader>
                  <Table.ColumnHeader textAlign="end">Price</Table.ColumnHeader>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {items.map((item) => (
                  <Table.Row key={item.id}>
                    <Table.Cell>{item.name}</Table.Cell>
                    <Table.Cell>{item.category}</Table.Cell>
                    <Table.Cell textAlign="end">{item.price}</Table.Cell>
                  </Table.Row>
                ))}
              </Table.Body>
            </Table.Root>

          </Stack>
        )}
      </>
    )
  }

  const scrollerUserTakingRecordComponent = () => (
    <>
      {pageModel !== undefined && (
        <Scroller
          title="Catatan"
        >
          {pageModel.takingRecords.length <= 0 && (<b>Data Kosong</b>)}

          {pageModel.takingRecords.length > 0 &&
            pageModel.takingRecords.map((record, index) => (
              <React.Fragment
                key={index}
              >
                {detailedCard(record)}
              </React.Fragment>
            ))
          }
        </Scroller>
      )}
    </>
  )

  const customerCalendar = () =>
    <Stack>
      <Calendar
        user={selectedCustomer}
        month={selectedDate}
      />
    </Stack>

  useEffect(() => {
    setListCustomer(activeCustomer)
    setHeaderInformation({
      title: 'Make Payment Page',
      description: 'place for customer to pay bill'
    })

    return () => setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }, [])

  useEffect(() => {

    if (selectedCustomer === undefined || selectedDate === undefined)
      setShowDetailTaking(false)

    if (selectedCustomer !== undefined && selectedDate !== undefined) {
      const serializedData = toFormData({
        userId: selectedCustomer.id,
        date: selectedDate,
        _action: ListActionEnum.GetPageModel
      })

      fetcher.submit(serializedData, {
        method: 'post',
      })
    }

  }, [selectedCustomer, selectedDate])

  useEffect(() => {

    if (fetcher.data !== undefined) {
      setPageModel(fetcher.data.pageModel)
      setShowDetailTaking(true)
    }

  }, [fetcher.data])


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

  return (
    <div className="make-payment-page">

      <Stack direction={'row'} className="detailed-info-container">
        <Stack className="left-container">

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

          <Heading>
            Pilih Bulan dan Tahun
          </Heading>

          <label>
            <DatePicker
              placeholderText="Bulan Dan Tahun"
              dateFormat="dd MMMM yyyy "
              selected={selectedDate}
              onChange={(date) => handleOnDatePickerChange(date)}
              showMonthYearPicker
              withPortal
              open={dateOpen}
              onClickOutside={() => setDateOpen(false)}
              onInputClick={() => setDateOpen(true)}
            />
          </label>

          {showDetailTaking && pageModel && selectedDate && (
            <Card.Root>
              <Card.Header>
                <Heading>
                  <Flex alignItems={'center'}>
                    <AiFillCalendar />
                    Informasi Bulan {formatDateId(selectedDate, "MMMM yyyy")}
                  </Flex>
                </Heading>
              </Card.Header>
              <Flex>
                <Box>
                  <Card.Header>
                    <Heading>
                      Total Ambil
                    </Heading>
                  </Card.Header>

                  <Card.Body>
                    <DataList.Root>

                      {dataListItemValue('Total', `${pageModel.detailInformation.totalAmount} Ampas`)}
                      {dataListItemValue('Terbayar', `${pageModel.detailInformation.paidAmount} Ampas`)}
                      {dataListItemValue('Belum Terbayar', `${pageModel.detailInformation.unpaidAmount} Ampas`)}

                    </DataList.Root>
                  </Card.Body>
                </Box>

                <Box>
                  <Card.Header>
                    <Heading>
                      Tagihan
                    </Heading>
                  </Card.Header>

                  <Card.Body>
                    <DataList.Root>

                      {dataListItemValue('Total', `${formatAsRupiah(pageModel.detailInformation.totalBill)}`)}
                      {dataListItemValue('Terbayar', `${formatAsRupiah(pageModel.detailInformation.paidBill)}`)}
                      {dataListItemValue('Belum Terbayar', `${formatAsRupiah(pageModel.detailInformation.unpaidBill)}`)}

                    </DataList.Root>
                  </Card.Body>
                </Box>
              </Flex>

              <Card.Footer>
                <Button
                  onClick={() => handleOnPayButtonClicked()}
                  disabled={isPayButtonDisabled()}
                >
                  <AiFillGolden />
                  Bayar Bulan {selectedDate && format(selectedDate, 'MMMM yyyy', { locale: id })}
                </Button>
              </Card.Footer>
            </Card.Root>
          )}

        </Stack>

        {showDetailTaking && pageModel !== undefined && (
          <Box className="tabs-container">
            <Tabs.Root defaultValue={ListTabEnum.CustomerTakingRecordDetail}>

              <Tabs.List>
                <Tabs.Trigger value={ListTabEnum.CustomerTakingRecordDetail}>
                  <AiFillSliders />
                  {ListTabEnum.CustomerTakingRecordDetail}
                </Tabs.Trigger>

                <Tabs.Trigger value={ListTabEnum.CustomerDetailInformation}>
                  <AiFillNotification />
                  {ListTabEnum.CustomerDetailInformation}
                </Tabs.Trigger>

                <Tabs.Trigger value={ListTabEnum.CustomerTakingCalendar}>
                  <AiFillCalendar />
                  {ListTabEnum.CustomerTakingCalendar}
                </Tabs.Trigger>
              </Tabs.List>

              <Tabs.Content value={ListTabEnum.CustomerTakingRecordDetail}>
                {scrollerUserTakingRecordComponent()}
              </Tabs.Content>

              <Tabs.Content value={ListTabEnum.CustomerDetailInformation}>
                {userDetailComponent()}
              </Tabs.Content>

              <Tabs.Content value={ListTabEnum.CustomerTakingCalendar}>
                {customerCalendar()}
              </Tabs.Content>

            </Tabs.Root>
          </Box>
        )}
      </Stack>
    </div>
  )
}
