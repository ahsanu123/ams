import { useAdminPageStore } from '@/state'
import './AdminPage.css'

export default function AdminPage() {

  const handleOnPrevClick = useAdminPageStore(state => state.decreaseYear)
  const handleOnNextClick = useAdminPageStore(state => state.increaseYear)
  const selectedYear = useAdminPageStore(state => state.selectedYear)

  const summaryOfYear = (
    <>
      <h4>Summary Of {selectedYear.getFullYear()}</h4>

      <label>
        label :
        <input
          value="something"
        />
      </label>

      <label>
        label :
        <input
          value="something"
        />
      </label>
    </>
  )

  const productPrice = (
    <>
      <h4>Product Price</h4>

      <label>
        label :
        <input
          value="something"
        />
      </label>

      <label>
        label :
        <input
          value="something"
        />
      </label>
    </>
  )

  const totalRevenue = (
    <>
      <h4>Total Revenue</h4>

      <label>
        label :
        <input
          value="something"
        />
      </label>

      <label>
        label :
        <input
          value="something"
        />
      </label>
    </>
  )

  const errorInformation = (
    <>
      <h4>Error Information</h4>

      <label>
        label :
        <input
          value="something"
        />
      </label>

      <label>
        label :
        <input
          value="something"
        />
      </label>
    </>
  )

  return (
    <div>
      <button
        onClick={handleOnPrevClick}
      >
        Prev Year
      </button>
      {" "}
      <button
        onClick={handleOnNextClick}
      >
        Next Year
      </button>
      <div
        className="admin-page"
      >
        <div
          className="left-content"
        >
          {summaryOfYear}
          {totalRevenue}
        </div>

        <div
          className="right-content"
        >
          {productPrice}
          {errorInformation}
        </div>
      </div>
    </div>
  )
}
