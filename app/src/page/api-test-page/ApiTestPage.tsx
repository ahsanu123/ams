import type { JSX } from "react"

export default function ApiTestPage() {

  const TestAddUserMoney = () => (
    <div className="test-add-customer-money">
      <p>test add customer money</p>
      <input />
    </div>
  )

  return (
    <>
      <h1>Api Test Page</h1>
      <TestAddUserMoney />
    </>
  )
}
