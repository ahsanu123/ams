import { useState } from "react"
import "./DregInputKeyboard.css"

type OkOrHapus = "Hapus" | "Ok"
const MAX_AMOUNT = 10

interface VirtualKeyboardProps {
  title?: string
  description?: string
  cancelText?: string
  confirmText?: string
  onOk: (amount: number) => void
}

export default function DregInputKeyboard(props: VirtualKeyboardProps) {

  const {
    title,
    description,
    cancelText = "Hapus",
    confirmText = "Ok",
    onOk
  } = props

  const [value, setValue] = useState<number>(0)
  const [warning, setWarning] = useState<string>()

  const keypad: (number | string)[] = [
    7, 8, 9, 4, 5, 6, 1, 2, 3, 0,
    cancelText, confirmText
  ]

  const handleOnNumberClick = (num: number) => {
    setWarning(undefined)
    if (value === undefined)
      setValue((num))

    else if (value + num > MAX_AMOUNT) return

    else setValue((value * 10 + num))
  }

  const handleOnCmdButtonClicked = (cmd: OkOrHapus) => {
    if (value === undefined) return

    if (cmd === 'Hapus') {
      const amountStr = value.toString()
      const amountNum = parseInt(amountStr.slice(0, -1))
      setValue(amountNum || 0)
    }

    if (cmd === 'Ok') {
      if (value === 0 || value === undefined) {
        setWarning("Jumlah Tidak Boleh Nol/0")
      }
      else
        onOk(value)
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
        value={value ?? 0}
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
