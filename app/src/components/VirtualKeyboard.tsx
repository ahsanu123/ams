import { useState } from "react"
import React from "react"
import "./VirtualKeyboard.css"


interface VirtualKeyboardProps {
  title?: string,
  description?: string
  cancelText?: string
  confirmText?: string
  handleOnConfirm: (value: string) => void
  handleOnCancel?: () => void
}

export default function VirtualKeyboard(props: VirtualKeyboardProps) {

  const {
    title,
    description,
    cancelText = "Hapus",
    confirmText = "Ok",
    handleOnConfirm,
    handleOnCancel
  } = props

  const [value, setValue] = useState<string>("")

  const handleOnKeyClicked = (localValue: string) => {
    if (localValue == 'Space')
      setValue(value.concat(" "))
    else
      setValue(value.concat(localValue))
  }

  const defaulthandleOnCancel = () => {
    if (value.length == 1) setValue('')
    setValue(value.slice(0, -1))
  }

  const keyboard: string[] = [
    "A", "B", "C", "D", "E", "F",
    "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R",
    "S", "T", "U", "V", "W", "X",
    "Y", "Z", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "0",
    "Space"
  ]

  return (
    <div
      className="virtual-keyboard-container"
    >
      <h5>{title}</h5>
      <sub>{description}</sub>

      <input
        id='virtual-keyboard-input'
        disabled
        type='text'
        value={value}
        onChange={(event) => handleOnKeyClicked(event.target.value)}
      />
      <div
        className="virtual-keyboard"
      >
        {keyboard.map((key, index) => (
          <React.Fragment
            key={index}
          >
            <button
              onClick={() => handleOnKeyClicked(key)}
            >
              {key}
            </button>
          </React.Fragment>
        ))}

        <br />
        <button
          className="cmd-button"
          onClick={() => handleOnCancel ? handleOnCancel() : defaulthandleOnCancel()}
        >
          {cancelText}
        </button>

        <button
          className="cmd-button"
          onClick={() => handleOnConfirm(value)}
        >
          {confirmText}
        </button>

      </div>
    </div>
  )
}
