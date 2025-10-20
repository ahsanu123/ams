import type { MakePaymentPageModel, TakingRecordWithPrice, UserModel } from "@/api-models"
import { makePaymentCommand, userManagementCommand } from "@/commands"
import Scroller from "@/component/Scroller"
import Tab from "@/component/Tab"
import { EMPTY_HEADER_INFORMATION } from "@/constants"
import { useMainLayoutStore } from "@/state"
import { formatAsRupiah, fromFormData, toFormData } from "@/utility"
import { format } from "date-fns"
import { id } from "date-fns/locale"
import React, { useEffect, useState } from "react"
import DatePicker from "react-datepicker"
import "react-datepicker/dist/react-datepicker.css"
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
  // FIXME: 
  // for now use if else, 
  // think better approach of it

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
  CustomerTakingRecordDetail = 'List Record',
  CustomerDetailInformation = 'Customer Information'
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
  const [activeTab, setActiveTab] = useState<string>(ListTabEnum.CustomerTakingRecordDetail)

  const handleOnDatePickerChange = (date: Date | null) => {
    if (date) {
      const utcTime = Date.UTC(date.getFullYear(), date.getMonth(), 1)
      setSelectedDate(new Date(utcTime))
      setDateOpen(false)
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

  const handleOnActiveTabChange = (title: string) => {
    setActiveTab(title)
  }

  const isPayButtonDisabled = () =>
    selectedDate === undefined
    || (pageModel && pageModel.takingRecords.length <= 0)
    || (pageModel && pageModel.takingRecords.every(pr => pr.takingRecord.isPaid === true))

  const detailedCard = (record: TakingRecordWithPrice) => (
    <div className={`detailed-card ${record.takingRecord.isPaid ? 'paid' : ''}`} >
      <div>
        <h2>{selectedCustomer?.username} {record.takingRecord.isPaid && " - Lunas"}</h2>
        <p>{format(record.takingRecord.takenDate, "PPPP", { locale: id })}</p>
        <p>{format(record.takingRecord.takenDate, "p", { locale: id })}</p>
      </div>

      <div className="record">
        <h2>{record.takingRecord.amount}</h2>
        <br />
        <b>Ampas</b>
      </div>
    </div >
  )

  const userDetailComponent = () => (
    <>
      {selectedCustomer !== undefined && (
        <div>
          <ul>
            <li>
              Nama: <b>{selectedCustomer.username}</b>
            </li>
            <li>
              {selectedCustomer.money >= 0 ? 'Memiliki Uang ' : 'Kekurangan Uang '} Sebesar:
              <b>{formatAsRupiah(selectedCustomer.money)}</b>
            </li>
          </ul>
        </div>
      )}
    </>
  )
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



  return (
    <div className="make-payment-page">

      <div className="detailed-info-container">
        <div>

          <label>
            Pilih Nama
            <br />
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
          </label>

          <label>
            Pilih Bulan dan Tahun
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

          {showDetailTaking && pageModel && (
            <>
              <h3>Total Ambil</h3>
              <ul>
                <li>
                  Total: <b>{pageModel.detailInformation.totalAmount} Ampas</b>
                </li>
                <li>
                  Terbayar: <b>{pageModel.detailInformation.paidAmount} Ampas</b>
                </li>
                <li>
                  Belum Terbayar: <b>{pageModel.detailInformation.unpaidAmount} Ampas</b>
                </li>
              </ul>

              <h3>Tagihan</h3>
              <ul>
                <li>
                  Total: <b>{formatAsRupiah(pageModel.detailInformation.totalBill)}</b>
                </li>
                <li>
                  Terbayar: <b>{formatAsRupiah(pageModel.detailInformation.paidBill)}</b>
                </li>
                <li>
                  Belum Terbayar: <b>{formatAsRupiah(pageModel.detailInformation.unpaidBill)}</b>
                </li>
              </ul>
              <br />

              <hr />
              <button
                onClick={() => handleOnPayButtonClicked()}
                disabled={isPayButtonDisabled()}
                className="pay-button"
              >
                Bayar Bulan {selectedDate && format(selectedDate, 'MMMM yyyy', { locale: id })}
              </button>
            </>
          )}

        </div>

        {showDetailTaking && pageModel !== undefined && (
          <div className="container-item">
            <Tab
              activeTab={activeTab}
              handleOnActiveTabChange={handleOnActiveTabChange}
              data={[
                {
                  title: ListTabEnum.CustomerTakingRecordDetail,
                  component: () => scrollerUserTakingRecordComponent(),
                },
                {
                  title: ListTabEnum.CustomerDetailInformation,
                  component: () => userDetailComponent()
                }
              ]}
            />
          </div>
        )}

      </div>
    </div>
  )
}
