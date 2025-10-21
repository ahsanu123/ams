import { useState } from "react"
import "./VirtualKeypad.css"
import React from "react"
import { Button, Heading } from "@chakra-ui/react"

type OkOrHapus = "Hapus" | "Ok"
const MAX_AMOUNT = 10

interface VirtualKeypadProps {
  title?: string,
  defaultValue?: number,
  description?: string
  cancelText?: string
  confirmText?: string
  inputType?: React.HTMLInputTypeAttribute
  handleOnConfirm: (value: number) => void
  validatorFunction?: (value: number) => string | undefined
}

export default function VirtualKeypad(props: VirtualKeypadProps) {

  const {
    title,
    defaultValue = 0,
    description,
    cancelText = "Hapus",
    confirmText = "Ok",
    inputType = 'number',
    handleOnConfirm,
    validatorFunction
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
    const validatorMessage = validatorFunction?.(value)

    if (value === undefined) return

    else if (cmd === 'Hapus') {
      const amountStr = value.toString()
      const amountNum = parseInt(amountStr.slice(0, -1))
      setValue(amountNum || 0)
    }

    else if (cmd === 'Ok' && validatorMessage != undefined) {
      setWarning(validatorMessage)
    }

    else if (cmd === 'Ok') {
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
            <Heading>
              {warning}
            </Heading>
          )
          : (
            <>
              <Heading>{title}</Heading>
              <sub>{description}</sub>
            </>
          )
      }

      <input
        id='virtual-keypad-input'
        disabled
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
                <Button
                  onClick={() => handleOnCmdButtonClicked(key as OkOrHapus)}
                >
                  {key}
                </Button>
              )
              : (
                <Button
                  onClick={() => handleOnNumberClick(key)}
                  colorPalette={'teal'}
                >
                  {key}
                </Button>
              )
            }
          </React.Fragment>
        ))}
      </div>
    </div>
  )
}
