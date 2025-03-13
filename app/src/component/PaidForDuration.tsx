import { defaultClient } from "../api/constant";
import { AmpasPricingService } from "../api/generated";
import { useEffect, useState } from "react";

interface PaidForDurationProps {
  username: string
  onSubmit?: () => void
}

interface PaidForDurationModel {
  from: Date,
  to: Date,
  username: string,
  status: PaidStatusType
}

type PaidStatusType = "unpaid" | "paid"

export default function PaidForDuration(props: PaidForDurationProps) {

  const {
    username,
    onSubmit,
  } = props

  const [paidDuration, setPaidDuration] = useState<PaidForDurationModel>();

  const handleOnFromChange = (date: Date) => {
    paidDuration && setPaidDuration({
      ...paidDuration,
      from: date
    })
  }

  const handleOnToChange = (date: Date) => {
    paidDuration && setPaidDuration({
      ...paidDuration,
      to: date
    })
  }

  const handleOnStatusSelected = (status: PaidStatusType) => {
    console.log("status: ", status)
    paidDuration && setPaidDuration({
      ...paidDuration,
      status
    })
  }

  const handleOnSubmit = async () => {
    console.log("paid duration: ", paidDuration)
    const result = await AmpasPricingService.postAmpasPricingSetPaidStatus({
      client: defaultClient,
      body: {
        username: paidDuration?.username,
        from: paidDuration?.from,
        to: paidDuration?.to,
        paid: paidDuration?.status === "paid"
      }
    })

    console.log("set paid status", result.data)
    onSubmit?.()
  }

  useEffect(() => {
    setPaidDuration({
      from: new Date(),
      to: new Date(),
      status: "paid",
      username
    })
  }, [username])

  return (
    <>
      <b>Set Paid Status For {username}</b>
      <div
        style={{ display: 'flex', gap: '20px' }}
      >
        <label>
          From
          <input
            type="date"
            onChange={(event) => handleOnFromChange(new Date(event.target.value))}
          />
        </label>
        <label>
          To
          <input
            type="date"
            onChange={(event) => handleOnToChange(new Date(event.target.value))}
          />
        </label>
        <select
          defaultValue={"paid"}
          onChange={(event) => handleOnStatusSelected(event.target.value as PaidStatusType)}
        >
          <option
            value="paid"
          >
            paid
          </option>
          <option
            value="unpaid"
          >
            unpaid
          </option>
        </select>

        <button
          onClick={() => handleOnSubmit()}
        >
          Submit
        </button>
      </div>
    </>
  )
}
