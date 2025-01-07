import { useEffect, useState } from "react"
import "./UserBillInformation.css"
import { AmpasDailyService, LocalAccountService, type AmpasSummary } from "../api/generated"
import { AmpasSummaryDurationEnum, defaultClient } from "../api/constant"
import { formatAsRupiah } from "../utility/format-as-rupiah"

export async function clientLoader() {
  const summaries: AmpasSummary[] = []

  const listUsername = await LocalAccountService.getLocalAccountListUser({
    client: defaultClient
  })

  const usernames = listUsername.data as string[]
  usernames.forEach(async (username) => {
    const summary = await AmpasDailyService.postAmpasDailyUserSummaryInfo({
      client: defaultClient,
      query: {
        duration: AmpasSummaryDurationEnum.Monthly,
        username,
      },
      body: new Date()
    })
    const data = summary.data
    data && summaries.push(data)
  })

  return summaries
}

interface RenderTableProps {
  title?: string,
  headData: Array<(string | number)[]>,
  bodyData: Array<(string | number)[]>,
}

const RenderTable = (props: RenderTableProps) => {

  const {
    title,
    headData,
    bodyData
  } = props

  return (
    <>
      <table
        title={title ?? ""}
      >
        <thead>
          {headData.map((container) => (
            <tr>
              {container.map((item) => (
                <td>
                  <b>
                    {item}
                  </b>
                </td>
              ))}
            </tr>
          ))}
        </thead>

        <tbody>
          {bodyData.map((container) => (
            <tr>
              {container.map((item) => (
                <td>
                  {item}
                </td>
              ))}
            </tr>
          ))}
        </tbody>

      </table>
    </>
  )
}

export default function UserBillInformation() {

  const [ampasSummary, setAmpasSummary] = useState<AmpasSummary[]>()

  const getAmpasSummary = async () => {

    const summaries: AmpasSummary[] = []

    const listUsername = await LocalAccountService.getLocalAccountListUser({
      client: defaultClient
    })

    const usernames = listUsername.data as string[]
    usernames.forEach(async (username) => {
      const summary = await AmpasDailyService.postAmpasDailyUserSummaryInfo({
        client: defaultClient,
        query: {
          duration: AmpasSummaryDurationEnum.Monthly,
          username,
        },
        body: new Date()
      })
      const data = summary.data
      data && summaries.push(data)
    })
    setAmpasSummary(summaries)
  }

  const data = ampasSummary?.map((item) => (
    [item.username ?? "", item.totalTaken ?? "", formatAsRupiah(item.totalTakenPrice ?? 0)]
  ))

  useEffect(() => {
    getAmpasSummary()
  }, [])

  return (
    <>
      {data && (
        <RenderTable
          headData={[
            ["username", "total taken", "total bill"]
          ]}
          bodyData={data}
        />
      )}
    </>
  )
}
