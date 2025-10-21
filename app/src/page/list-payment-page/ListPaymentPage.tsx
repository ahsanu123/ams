import Scroller from "@/component/Scroller";
import { useMainLayoutStore } from "@/state";
import { useEffect, useState } from "react";
import DatePicker from "react-datepicker";
import { useListPaymentPageState } from "./list-payment-page-state";
import './ListPaymentPage.css';
import type { PaymentHistoryModel, TakingRecordModel } from "@/api-models";
import type { Route } from "./+types/ListPaymentPage";
import { useFetcher } from "react-router";
import { paymentHistoryCommand, takingRecordCommand, userManagementCommand } from "@/commands";
import { EMPTY_HEADER_INFORMATION } from "@/constants";
import { formatAsRupiah, fromFormData, toFormData } from "@/utility";
import { format } from "date-fns";
import { id } from "date-fns/locale";
import React from "react";

enum IClientActionType {
  GetRecordByUserId,
  GetRecordByMonth,
  GetRecordByUserIdAndMonth
}

interface IClientActionRequestModel {
  userId: number,
  date: Date,
  _action: IClientActionType,
}

export async function clientLoader() {
  const activeCustomer = await userManagementCommand.getAllActiveUser()
  return {
    activeCustomer
  }
}

export async function clientAction({ request }: Route.ClientActionArgs): Promise<PaymentHistoryModel[]> {
  const parsedRequest = await fromFormData<IClientActionRequestModel>(request)
  let records: PaymentHistoryModel[] = [];

  // if (parsedRequest._action === IClientActionType.GetRecordByUserId) {
  // }
  // else if (parsedRequest._action === IClientActionType.GetRecordByMonth) {
  // }
  //
  // else if (parsedRequest._action === IClientActionType.GetRecordByUserIdAndMonth) {
  records = await paymentHistoryCommand.getPaymentRecordByUserIdAndMonth(parsedRequest.userId, parsedRequest.date)
  // }

  return records
}


export default function ListPaymentPage({
  loaderData
}: Route.ComponentProps) {

  const { activeCustomer } = loaderData
  const fetcher = useFetcher<PaymentHistoryModel[]>()

  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)

  const listCustomer = useListPaymentPageState(state => state.listCustomer)
  const setListCustomer = useListPaymentPageState(state => state.setListCustomer)

  const selectedCustomer = useListPaymentPageState(state => state.selectedCustomer)
  const setSelectedCustomer = useListPaymentPageState(state => state.setSelectedCustomer)

  const selectedDate = useListPaymentPageState(state => state.selectedDate)
  const setSelectedDate = useListPaymentPageState(state => state.setSelectedDate)

  const paymentRecords = useListPaymentPageState(state => state.paymentRecords)
  const setPaymentRecords = useListPaymentPageState(state => state.setPaymentRecords)

  const [dateOpen, setDateOpen] = useState(false)

  const handleOnDatePickerChange = (date: Date | null) => {
    if (date) {
      const utcTime = Date.UTC(date.getFullYear(), date.getMonth(), 1)
      setSelectedDate(new Date(utcTime))
      setDateOpen(false)
    }
  }

  const detailedCard = (record: PaymentHistoryModel) => (
    <div className='detailed-card' >
      <div>
        <h4>{selectedCustomer?.username}</h4>
        <p>{format(record.date, "PPPP", { locale: id })}</p>
      </div>

      <div className="record">
        <h2>{record.billAmount}</h2>
        <br />
        <b>Ampas</b>
      </div>
    </div >
  )

  useEffect(() => {
    // if (selectedCustomer !== undefined) {
    //   const serializedData = toFormData({
    //     userId: selectedCustomer.id,
    //     _action: IClientActionType.GetRecordByUserId
    //   })
    //
    //   fetcher.submit(serializedData, {
    //     method: 'post',
    //   })
    // }

    if (selectedCustomer !== undefined && selectedDate !== undefined) {
      const serializedData = toFormData({
        userId: selectedCustomer.id,
        date: selectedDate,
        _action: IClientActionType.GetRecordByUserIdAndMonth
      })

      fetcher.submit(serializedData, {
        method: 'post',
      })
    }

    // else if (selectedDate !== undefined) {
    //   const serializedData = toFormData({
    //     date: selectedDate,
    //     _action: IClientActionType.GetRecordByMonth
    //   })
    //
    //   fetcher.submit(serializedData, {
    //     method: 'post',
    //   })
    // }

  }, [selectedCustomer, selectedDate])

  useEffect(() => {
    setListCustomer(activeCustomer)
    setHeaderInformation({
      title: 'List Payment',
      description: 'Place To See All Taking Record'
    })

    return () => setHeaderInformation(EMPTY_HEADER_INFORMATION)
  }, [])

  useEffect(() => {
    if (fetcher.data !== undefined)
      setPaymentRecords(fetcher.data)
  }, [fetcher.data])

  return (
    <div className="list-payment-page">

      <div className="name-and-date-picker">
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
      </div>

      <div>
        {
          selectedDate && selectedCustomer && (
            <Scroller
              title="Catatan Pembayaran"
            >
              {paymentRecords.length <= 0 && (<b>Data Kosong</b>)}

              {paymentRecords.length > 0 &&
                paymentRecords.map((record, index) => (
                  <React.Fragment
                    key={index}
                  >
                    {detailedCard(record)}
                  </React.Fragment>
                ))
              }
            </Scroller>
          )
        }
      </div>
    </div>
  )
}
