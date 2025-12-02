import type { DregPriceModel, MakePaymentPageModel, RangePaymentInfo, TakingRecordWithPrice, UserModel } from "@/api-models"
import { dregPriceCommand, makePaymentCommand, takingRecordCommand, userManagementCommand } from "@/commands"
import Calendar from "@/component/Calendar"
import Scroller from "@/component/Scroller"
import { EMPTY_HEADER_INFORMATION } from "@/constants"
import { useMainLayoutStore } from "@/state"
import { formatAsRupiah, formatDateId, fromFormData, toFormData } from "@/utility"
import { Avatar, Badge, Box, Button, Card, createListCollection, DataList, Flex, Heading, Portal, Select, Stack, Table, Tabs, Text } from "@chakra-ui/react"
import { compareAsc, eachMonthOfInterval, format, isSameMonth, setMonth } from "date-fns"
import { id } from "date-fns/locale"
import React, { useEffect, useState } from "react"
import DatePicker, { registerLocale } from "react-datepicker"
import { AiFillCalendar, AiFillGolden, AiFillNotification, AiFillSliders } from "react-icons/ai"
import { useFetcher } from "react-router"
import type { Route } from "./+types/MakePaymentPages"
import { useMakePaymentPageState } from "./make-payment-page-state"
import './MakePaymentPages.css'

// FIXME: 
// simplify and refactor this page

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

  const selectListCustomer = createListCollection({
    items: listCustomer.map((customer) => ({
      label: customer.username,
      value: customer.id.toString()
    }))
  })

  // TODO: move this to state 
  const [dregPrices, setDregPrices] = useState<DregPriceModel[]>([])
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

  const rangeRecordsTab = () => {
    if (!(fromDate && toDate && rangeRecords)) return;

    const rangeMonths = eachMonthOfInterval({ start: fromDate, end: toDate })

    return (
      <Stack maxWidth={'900px'}>
        <Tabs.Root
          orientation='vertical'
          defaultValue={formatDateId(fromDate, "MMMM")}>
          <Tabs.List>
            {
              rangeMonths.map((month) =>
                <Tabs.Trigger value={formatDateId(month, "MMMM")}>
                  {formatDateId(month, "MMMM")}
                </Tabs.Trigger>
              )
            }
          </Tabs.List>

          {
            rangeMonths.map((month) =>
              <Tabs.Content value={formatDateId(month, "MMMM")}>
                <Scroller>
                  <Table.Root
                    striped
                    stickyHeader
                    strokeLinecap={'round'}
                    size="lg"
                    variant={'line'}>

                    <Table.Header>
                      <Table.Row>
                        <Table.ColumnHeader>Jumlah</Table.ColumnHeader>
                        <Table.ColumnHeader>Status</Table.ColumnHeader>
                        <Table.ColumnHeader>Tanggal</Table.ColumnHeader>
                      </Table.Row>
                    </Table.Header>

                    <Table.Body>

                      Total
                      {' '}
                      {
                        rangeRecords.recordWithPrice
                          .filter(pr => isSameMonth(pr.takingRecord.takenDate, month))
                          .map(record => record.takingRecord.amount)
                          .reduce((p, c) => p + c, 0)
                      }
                      {' '} | {' '}
                      {
                        formatAsRupiah(
                          rangeRecords.recordWithPrice
                            .filter(pr => isSameMonth(pr.takingRecord.takenDate, month))
                            .map(record => record.takingRecord.amount * record.price.price)
                            .reduce((p, c) => p + c, 0)
                        )
                      }

                      {
                        rangeRecords.recordWithPrice
                          .filter(pr => isSameMonth(pr.takingRecord.takenDate, month))
                          .sort((p, n) => compareAsc(p.takingRecord.takenDate, n.takingRecord.takenDate))
                          .map(record =>

                            <Table.Row>
                              <Table.Cell>
                                {record.takingRecord.amount}
                              </Table.Cell>

                              <Table.Cell>
                                <Text>
                                  {formatAsRupiah(record.price.price)}
                                </Text>

                                {record.takingRecord.isPaid && (
                                  <Badge colorPalette={'green'}>
                                    Lunas
                                  </Badge>
                                )}
                              </Table.Cell>

                              <Table.Cell>
                                {formatDateId(record.takingRecord.takenDate)}
                              </Table.Cell>
                            </Table.Row>
                          )
                      }
                    </Table.Body>

                  </Table.Root>
                </Scroller>
              </Tabs.Content>
            )
          }

        </Tabs.Root>
      </Stack>
    )
  }

  const handleOnPayRangeButtonClicked = () => {
    if (!fromDate || !toDate || !selectedCustomer) return

    const rangeMonths = eachMonthOfInterval({ start: fromDate, end: toDate })
    const rangePaymentPromises = rangeMonths.map(month =>
      makePaymentCommand.makePayment(selectedCustomer.id, month)
    )

    Promise.all(rangePaymentPromises).then(_ =>
      userManagementCommand.getById(selectedCustomer.id).then(customer => setSelectedCustomer(customer))
    )
  }

  const isPayRangeButtonDisabled = () =>
    toDate === undefined
    || fromDate === undefined

  const isPayButtonDisabled = () =>
    selectedDate === undefined
    || (pageModel && pageModel.takingRecords.length <= 0)
    || (pageModel && pageModel.takingRecords.every(pr => pr.takingRecord.isPaid === true))

  const detailedCard = (record: TakingRecordWithPrice) => (
    <Card.Root className={`detailed-card ${record.takingRecord.isPaid ? 'paid' : ''}`} >
      <Card.Header>
        <Flex className="card-header">
          <Heading>
            <Flex gap={'15px'} alignItems={'center'}>
              <Avatar.Root>
                <Avatar.Fallback name={selectedCustomer?.username} />
              </Avatar.Root>

              <Text>{selectedCustomer?.username}</Text>
            </Flex>
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
                    <Table.ColumnHeader>Harga Satuan</Table.ColumnHeader>
                    <Table.ColumnHeader>Harga Ambil</Table.ColumnHeader>
                  </Table.Row>
                </Table.Header>
                <Table.Body>
                  {
                    oneYearRecords
                      .map((record) => (
                        <Table.Row key={record.id}>
                          <Table.Cell>{formatDateId(record.takenDate)}</Table.Cell>
                          <Table.Cell>{record.amount}</Table.Cell>
                          <Table.Cell>{formatAsRupiah(dregPrices.find(pr => pr.id === record.priceId)?.price ?? 0)}</Table.Cell>
                          <Table.Cell>{formatAsRupiah((dregPrices.find(pr => pr.id === record.priceId)?.price ?? 0) * record.amount)}</Table.Cell>
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
          minHeight='700px'
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
        listMonth.map((date, index) =>
          <React.Fragment key={index}>
            <Calendar
              user={selectedCustomer}
              month={date}
            />
          </React.Fragment>
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
    <Stack>
      <Flex justifyContent={'space-between'}>
        <Box width={'100%'}>
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
        </Box>

        <Box width={'100%'}>
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
        </Box>
      </Flex>

      {
        !rangeRecords
        && fromDate
        && toDate
        && <Heading>
          Tidak ada data dari {formatDateId(fromDate, "MMMM yyyy")} hingga {formatDateId(toDate, "MMMM yyyy")}
        </Heading>
      }

      {
        fromDate && toDate && rangeRecords && (
          <Flex>
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
                <Flex>
                  <Box>
                    <Card.Header>
                      <Heading>
                        Total Ambil
                      </Heading>
                    </Card.Header>

                    <Card.Body>
                      <DataList.Root>

                        {dataListItemValue('Total', `${rangeRecords.detailInformation.totalAmount} Ampas`)}
                        {dataListItemValue('Terbayar', `${rangeRecords.detailInformation.paidAmount} Ampas`)}
                        {dataListItemValue('Belum Terbayar', `${rangeRecords.detailInformation.unpaidAmount} Ampas`)}

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

                        {dataListItemValue('Total', `${formatAsRupiah(rangeRecords.detailInformation.totalBill)}`)}
                        {dataListItemValue('Terbayar', `${formatAsRupiah(rangeRecords.detailInformation.paidBill)}`)}
                        {dataListItemValue('Belum Terbayar', `${formatAsRupiah(rangeRecords.detailInformation.unpaidBill)}`)}

                      </DataList.Root>
                    </Card.Body>
                  </Box>
                </Flex>
              </Card.Body>

              <Card.Footer>
                <Button
                  onClick={() => handleOnPayRangeButtonClicked()}
                  disabled={isPayRangeButtonDisabled()}
                >
                  Bayar {formatDateId(rangeRecords.from, "MMMM yyyy")}
                  {' '}
                  Hingga
                  {' '}
                  {formatDateId(rangeRecords.to, "MMMM yyyy")}
                </Button>
              </Card.Footer>

            </Card.Root>

            {rangeRecordsTab()}
          </Flex>
        )
      }

    </Stack>

  const detailOneMonth = () => {
    return showDetailTaking && pageModel && selectedDate && (
      <Card.Root>
        <Card.Header>
          <Heading>
            <Flex alignItems={'center'}>
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

  const getRangeRecords = () => {
    if (selectedCustomer && fromDate && toDate)
      takingRecordCommand.getTakingRecordByUserIdAndRangeMonth(selectedCustomer.id, fromDate, toDate)
        .then((records) => setRangeRecords(records))
  }

  useEffect(() => {
    getRangeRecords()

  }, [fromDate, toDate, selectedCustomer])

  useEffect(() => {
    dregPriceCommand.getAllDregPrice().then(prices => setDregPrices(prices))

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
    getRangeRecords()
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

          <Select.Root
            size={'lg'}
            value={selectedCustomer ? [selectedCustomer.id.toString()] : []}
            collection={selectListCustomer}
            onValueChange={(e) => {
              if (e.value[0] !== undefined)
                setSelectedCustomer(
                  listCustomer.find(pr => pr.id === Number(e.value[0]))
                )
            }}>

            <Select.HiddenSelect />

            <Select.Control>
              <Select.Trigger textStyle={'lg'}>
                <Select.ValueText placeholder="Pilih Nama" />
              </Select.Trigger>
            </Select.Control>

            <Portal>

              <Select.Positioner>
                <Select.Content>
                  {
                    selectListCustomer.items.map((item) => (
                      <Select.Item
                        item={item}
                        key={item.value}>

                        <Text textStyle={'lg'}>{item.label}</Text>
                        <Select.ItemIndicator />

                      </Select.Item>
                    ))
                  }
                </Select.Content>
              </Select.Positioner>

            </Portal>

          </Select.Root>

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
