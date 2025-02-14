import { useState } from "react"
import "./VirtualKeyboard.css"

type OkOrHapus = "Hapus" | "Ok"
const MAX_AMOUNT = 10

interface VirtualKeyboardProps {
  title?: string
  description?: string
  onOk: (amount: number) => void
}

export default function VirtualKeyboard(props: VirtualKeyboardProps) {

  const {
    title,
    description,
    onOk
  } = props

  const [amount, setAmount] = useState<number>(0)
  const [warning, setWarning] = useState<string>()

  const keypad: (number | string)[] = [
    7, 8, 9, 4, 5, 6, 1, 2, 3, 0,
    "Hapus", "Ok"
  ]

  const handleOnNumberClick = (num: number) => {
    setWarning(undefined)
    if (amount === undefined)
      setAmount((num))

    else if (amount + num > MAX_AMOUNT) return

    else setAmount((amount * 10 + num))
  }

  const handleOnCmdButtonClicked = (cmd: OkOrHapus) => {
    if (amount === undefined) return

    if (cmd === 'Hapus') {
      const amountStr = amount.toString()
      const amountNum = parseInt(amountStr.slice(0, -1))
      setAmount(amountNum || 0)
    }

    if (cmd === 'Ok') {
      console.log(amount)
      if (amount === 0 || amount === undefined) {
        setWarning("Jumlah Tidak Boleh Nol/0")
      }
      else
        onOk(amount)
    }

  }

  return (
    <div
      className="virtual-keyboard-container"
    >
      {
        !!warning
          ? (
            <b
              className="stripe"
            >
              {warning}
            </b>
          )
          : (
            <>
              <h5>⭐{title}</h5>
              <sub>{description}</sub>
            </>
          )
      }


      <input
        type="number"
        value={amount ?? 0}
        onChange={(event) => handleOnNumberClick(parseInt(event.target.value))}
      />
      <div
        className="virtual-keyboard"
      >
        {keypad.map((key, index) => (
          <>
            {(typeof (key) === 'string')
              ? (
                <button
                  className="cmd-button"
                  onClick={() => handleOnCmdButtonClicked(key as OkOrHapus)}
                  key={`virtual-keyboard-cmd-${index}`}
                >
                  {key}
                </button>
              )
              : (
                <button
                  onClick={() => handleOnNumberClick(key)}
                  key={`virtual-keyboard-btn-${index}`}
                >
                  {key}
                </button>
              )
            }
          </>
        ))}
      </div>
    </div>
  )
}
