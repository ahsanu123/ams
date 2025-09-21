import { useState } from "react"
import "./VirtualKeypad.css"
import React from "react"

type OkOrHapus = "Hapus" | "Ok"
const MAX_AMOUNT = 10

interface VirtualKeyboardProps {
  title?: string,
  defaultValue?: number,
  description?: string
  cancelText?: string
  confirmText?: string
  inputType?: React.HTMLInputTypeAttribute
  handleOnConfirm: (value: number) => void
}

export default function VirtualKeypad(props: VirtualKeyboardProps) {

  const {
    title,
    defaultValue = 0,
    description,
    cancelText = "Hapus",
    confirmText = "Ok",
    inputType = 'number',
    handleOnConfirm
  } = props

  const [value, setValue] = useState<number>(defaultValue)
  const [warning, setWarning] = useState<string>()

  const keypad: (number | string)[] = [
    7, 8, 9, 4, 5, 6, 1, 2, 3, 0,
    "Hapus", "Ok"
  ]

  const handleOnNumberClick = (num: number) => {
    setValue((value * 10 + num))
  }

  const handleOnCmdButtonClicked = (cmd: OkOrHapus) => {
    if (value === undefined) return

    if (cmd === 'Hapus') {
      const amountStr = value.toString()
      const amountNum = parseInt(amountStr.slice(0, -1))
      setValue(amountNum || 0)
    }

    if (cmd === 'Ok') {
      handleOnConfirm(value)
    }

  }

  return (
    <div
      className="virtual-keypad-container"
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
              <h5>{title}</h5>
              <sub>{description}</sub>
            </>
          )
      }

      <input
        id='virtual-keypad-input'
        type={inputType}
        value={value ?? 0}
        onChange={(event) => handleOnNumberClick(parseInt(event.target.value))}
      />
      <div
        className="virtual-keypad"
      >
        {keypad.map((key, index) => (
          <React.Fragment
            key={index}
          >
            {(typeof (key) === 'string')
              ? (
                <button
                  className="cmd-button"
                  onClick={() => handleOnCmdButtonClicked(key as OkOrHapus)}
                >
                  {key}
                </button>
              )
              : (
                <button
                  onClick={() => handleOnNumberClick(key)}
                >
                  {key}
                </button>
              )
            }
          </React.Fragment>
        ))}
      </div>
    </div>
  )
}
