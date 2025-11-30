import type { MakePaymentPageModel, RangePaymentInfo, TakingRecordWithPrice, UserModel } from "@/api-models"
import { makePaymentCommand, takingRecordCommand, userManagementCommand } from "@/commands"
import Calendar from "@/component/Calendar"
import Scroller from "@/component/Scroller"
import { EMPTY_HEADER_INFORMATION } from "@/constants"
import { useMainLayoutStore } from "@/state"
import { formatAsRupiah, formatDateId, fromFormData, toFormData } from "@/utility"
import { Badge, Box, Button, Card, DataList, Flex, Heading, Stack, Table, Tabs, Text } from "@chakra-ui/react"
import { format, setMonth } from "date-fns"
import { id } from "date-fns/locale"
import React, { useEffect, useState } from "react"
import DatePicker, { registerLocale } from "react-datepicker"
import { AiFillCalendar, AiFillGolden, AiFillNotification, AiFillSliders } from "react-icons/ai"
import { useFetcher } from "react-router"
import type { Route } from "./+types/MakePaymentPages"
import { useMakePaymentPageState } from "./make-payment-page-state"
import './MakePaymentPages.css'

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
  CustomerTakingCalendar = 'Kalender',
  AllYearCalendar = 'Kalendar Satu Tahun',
  RangeMonthInformation = 'Informasi Beberapa Bulan',
}

export default function MakePaymentPage({
  loaderData
}: Route.ComponentProps) {

  const { activeCustomer } = loaderData

  registerLocale('id', id)

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

  const oneYearRecords = useMakePaymentPageState(state => state.oneYearRecords)
  const setOneYearRecords = useMakePaymentPageState(state => state.setOneYearRecrods)

  // TODO: move this to state 
  const [fromDate, setFromDate] = useState<Date | undefined>(undefined)
  const [toDate, setToDate] = useState<Date | undefined>(undefined)
  const [dateOpen, setDateOpen] = useState(false)
  const [fromDateOpen, setFromDateOpen] = useState(false)
  const [toDateOpen, setToDateOpen] = useState(false)
  const [activeTab, setActiveTab] = useState<string>(ListTabEnum.CustomerTakingRecordDetail)
  const [rangeRecords, setRangeRecords] = useState<RangePaymentInfo | undefined>()

  const listMonth = Array.from({ length: 12 }, (_, index) => index)
    .map((monthIndex) => setMonth(new Date(), monthIndex))


  const handleOnDatePickerChange = (date: Date | null) => {
    if (date) {
      const utcTime = Date.UTC(date.getFullYear(), date.getMonth(), 1)
      setSelectedDate(new Date(utcTime))
      setDateOpen(false)
    }
  }

  const handleOnFromDateChange = (date: Date | null) => {
    if (date) {
      const utcTime = Date.UTC(date.getFullYear(), date.getMonth(), 1)
      setFromDate(new Date(utcTime))
      setFromDateOpen(false)
    }
  }

  const handleOnToDateChange = (date: Date | null) => {
    if (date) {
      const utcTime = Date.UTC(date.getFullYear(), date.getMonth(), 1)
      setToDate(new Date(utcTime))
      setToDateOpen(false)
    }
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
      </Card.Body>

    </Card.Root>
  )

  const userDetailComponent = () => {
    const customerData = pageModel?.customers.find(pr => pr.id === selectedCustomer?.id)
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

            <Heading>
              Catatan Ambil Tahun {selectedDate?.getFullYear()}
              {' '}
              (Total {oneYearRecords.map(item => item.amount).reduce((p, n) => (p + n), 0)})
            </Heading>

            <Scroller>
              <Table.Root size="sm">
                <Table.Header>
                  <Table.Row>
                    <Table.ColumnHeader>Date</Table.ColumnHeader>
                    <Table.ColumnHeader>Amount</Table.ColumnHeader>
                  </Table.Row>
                </Table.Header>
                <Table.Body>
                  {
                    oneYearRecords
                      .map((record) => (
                        <Table.Row key={record.id}>
                          <Table.Cell>{formatDateId(record.takenDate)}</Table.Cell>
                          <Table.Cell>{record.amount}</Table.Cell>
                        </Table.Row>
                      ))}
                </Table.Body>
              </Table.Root>
            </Scroller>

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

  const allYearCalender = () =>
    <Scroller
      minHeight="70vh">
      {
        listMonth.map((date) =>
          <Calendar
            user={selectedCustomer}
            month={date}
          />
        )
      }
    </Scroller>

  const currentMonthCalender = () =>
    <Stack>
      <Calendar
        user={selectedCustomer}
        month={selectedDate}
      />
    </Stack>

  const rangeMonthInformation = () =>
    <Stack maxWidth={'500px'}>
      <Heading>Dari Bulan</Heading>
      <label>
        <DatePicker
          placeholderText="Bulan Dan Tahun"
          dateFormat="MMMM yyyy "
          selected={fromDate}
          onChange={(date) => handleOnFromDateChange(date)}
          showMonthYearPicker
          locale={'id'}
          withPortal
          open={fromDateOpen}
          onClickOutside={() => setFromDateOpen(false)}
          onInputClick={() => setFromDateOpen(true)}
        />
      </label>

      <Heading>Sampai Bulan</Heading>
      <label>
        <DatePicker
          placeholderText="Bulan Dan Tahun"
          dateFormat="MMMM yyyy "
          selected={toDate}
          onChange={(date) => handleOnToDateChange(date)}
          showMonthYearPicker
          locale={'id'}
          withPortal
          open={toDateOpen}
          onClickOutside={() => setToDateOpen(false)}
          onInputClick={() => setToDateOpen(true)}
        />
      </label>

      {
        rangeRecords
        && fromDate
        && toDate
        && <Heading>
          Tidak ada data dari {formatDateId(fromDate, "MMMM yyyy")} hingga {formatDateId(toDate, "MMMM yyyy")}
        </Heading>
      }
      {
        fromDate && toDate && rangeRecords && (
          <Card.Root>
            <Card.Header>
              <Heading>
                Informasi {formatDateId(rangeRecords.from, "MMMM yyyy")}
                {' '}
                Hingga
                {' '}
                {formatDateId(rangeRecords.to, "MMMM yyyy")}
              </Heading>
            </Card.Header>

            <Card.Body>
              <DataList.Root>

                {dataListItemValue('Total', `${rangeRecords.detailInformation.totalAmount} Ampas`)}
                {dataListItemValue('Terbayar', `${rangeRecords.detailInformation.paidAmount} Ampas`)}
                {dataListItemValue('Belum Terbayar', `${rangeRecords.detailInformation.unpaidAmount} Ampas`)}

              </DataList.Root>
            </Card.Body>

          </Card.Root>
        )
      }

    </Stack>

  const detailOneMonth = () => {
    return showDetailTaking && pageModel && selectedDate && (
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
    )
  }

  useEffect(() => {
    if (selectedCustomer && fromDate && toDate)
      takingRecordCommand.getTakingRecordByUserIdAndRangeMonth(selectedCustomer.id, fromDate, toDate)
        .then((records) => setRangeRecords(records))

  }, [fromDate, toDate, selectedCustomer])

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
      takingRecordCommand.getTakingRecordByUserIdAndYear(selectedCustomer.id, selectedDate)
        .then((records) => setOneYearRecords(records))

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
              dateFormat="MMMM yyyy "
              selected={selectedDate}
              onChange={(date) => handleOnDatePickerChange(date)}
              showMonthYearPicker
              locale={'id'}
              withPortal
              open={dateOpen}
              onClickOutside={() => setDateOpen(false)}
              onInputClick={() => setDateOpen(true)}
            />
          </label>

          {detailOneMonth()}

        </Stack>

        {showDetailTaking && pageModel !== undefined && (
          <Box className="tabs-container">
            <Tabs.Root
              value={activeTab}
              onValueChange={(e) => setActiveTab(e.value)}
              defaultValue={ListTabEnum.CustomerTakingRecordDetail}>

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

                <Tabs.Trigger value={ListTabEnum.AllYearCalendar}>
                  <AiFillCalendar />
                  {ListTabEnum.AllYearCalendar}
                </Tabs.Trigger>

                <Tabs.Trigger value={ListTabEnum.RangeMonthInformation}>
                  <AiFillCalendar />
                  {ListTabEnum.RangeMonthInformation}
                </Tabs.Trigger>
              </Tabs.List>

              <Tabs.Content value={ListTabEnum.CustomerTakingRecordDetail}>
                {scrollerUserTakingRecordComponent()}
              </Tabs.Content>

              <Tabs.Content value={ListTabEnum.CustomerDetailInformation}>
                {userDetailComponent()}
              </Tabs.Content>

              <Tabs.Content value={ListTabEnum.CustomerTakingCalendar}>
                {currentMonthCalender()}
              </Tabs.Content>

              <Tabs.Content value={ListTabEnum.AllYearCalendar}>
                {allYearCalender()}
              </Tabs.Content>

              <Tabs.Content value={ListTabEnum.RangeMonthInformation}>
                {rangeMonthInformation()}
              </Tabs.Content>

            </Tabs.Root>
          </Box>
        )}
      </Stack>
    </div>
  )
}
