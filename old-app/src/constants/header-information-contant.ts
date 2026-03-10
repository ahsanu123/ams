import type { HeaderInformation } from "@/model"

export const EMPTY_HEADER_INFORMATION: HeaderInformation = {
  title: '',
  description: ''
}

export const LOGGED_ADMIN_INFORMATION_MESSAGE: HeaderInformation = {
  title: 'Welcome Admin',
  description: 'you now able to do admin operation'
}

export const NOT_LOGGED_ADMIN_INFORMATION_MESSAGE: HeaderInformation = {
  title: 'Admin Only',
  description: 'you must login!'
}
