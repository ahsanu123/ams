

export async function get(url: string, requestInit?: RequestInit) {
  const defaultRequestInit: RequestInit = {
    method: 'GET',
  }
  return fetch(url, {
    ...requestInit,
    ...defaultRequestInit
  })
}

export async function post(url: string, requestInit?: RequestInit) {
  const defaultRequestInit: RequestInit = {
    method: 'POST',
  }
  return fetch(url, {
    ...requestInit,
    ...defaultRequestInit
  })
}
