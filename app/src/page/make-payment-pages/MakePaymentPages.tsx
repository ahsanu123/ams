import { EMPTY_HEADER_INFORMATION } from "@/constants"
import { useMainLayoutStore } from "@/state"
import { useEffect, useState } from "react"
import Scroller from "@/component/Scroller"
import DatePicker from "react-datepicker"
import "react-datepicker/dist/react-datepicker.css";
import './MakePaymentPages.css'
import { formatAsRupiah, fromFormData, toFormData } from "@/utility"
import { useMakePaymentPageState } from "./make-payment-page-state"
import { format } from "date-fns"
import { id } from "date-fns/locale"
import type { Route } from "./+types/MakePaymentPages"
import { makePaymentCommand, userManagementCommand } from "@/commands"
import { useFetcher } from "react-router"
import type { MakePaymentPageModel, TakingRecordWithPrice } from "@/api-models"
import React from "react"

interface IGetPageModelClientRequest {
  userId: number,
  date: Date
}

export async function clientLoader() {
  const activeCustomer = await userManagementCommand.getAllActiveUser()
  return {
    activeCustomer
  }
}

export async function clientAction({ request }: Route.ClientActionArgs): Promise<MakePaymentPageModel> {
  const data = await fromFormData<IGetPageModelClientRequest>(request)
  const pageModel = await makePaymentCommand.getPageModel(data.userId, data.date)

  return pageModel
}

export default function MakePaymentPage({
  loaderData
}: Route.ComponentProps) {

  const { activeCustomer } = loaderData

  const fetcher = useFetcher<MakePaymentPageModel>()

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

  const handleOnPayButtonClicked = () => {
    // TODO:

    console.log(selectedCustomer, selectedDate)
  }

  const isPayButtonDisabled = () =>
    selectedDate === undefined
    || (pageModel && pageModel.takingRecords.length <= 0)
    || (pageModel && pageModel.takingRecords.every(pr => pr.takingRecord.isPaid === true))

  const detailedCard = (record: TakingRecordWithPrice) => (
    <div className="detailed-card">
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
    </div>
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
        date: selectedDate
      })

      fetcher.submit(serializedData, {
        method: 'post'
      })
    }

  }, [selectedCustomer, selectedDate])

  useEffect(() => {

    if (fetcher.data !== undefined) {
      setPageModel(fetcher.data)
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
              <b>{pageModel.detailInformation.takingCountForCurrentMonth} Ampas</b>

              <h3>Tagihan</h3>
              <b>{formatAsRupiah(pageModel.detailInformation.totalBillForCurrentMonth)}</b>
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
          <div>
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
          </div>
        )}

      </div>
    </div>
  )
}
