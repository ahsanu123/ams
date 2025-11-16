import type { KeyboardLayoutObject } from "react-simple-keyboard"

// https://hodgef.com/simple-keyboard/documentation/options/layout/
export const textKeyboardLayout: KeyboardLayoutObject = {
  'default': [
    "q w e r t y u i o p",
    "a s d f g h j k l",
    "z x c v b n m",
    "{space} {bksp}"
  ]
}

export const textOrNumberKeyboardDisplay: { [key: string]: string } = {
  "{space}": "Space",
  "{enter}": "Enter",
  "{bksp}": "Backspace"
}

export const numberLayout: KeyboardLayoutObject = {
  'default': [
    "7 8 9",
    "4 5 6",
    "1 2 3",
    "0 {bksp} {enter}"
  ]
}

