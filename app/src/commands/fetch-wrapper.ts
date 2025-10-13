export async function asJson<T>(response: Response): Promise<T> {
  const json = await response.json() as T
  return json
}

export async function get(url: string, requestInit?: RequestInit) {
  const defaultRequestInit: RequestInit = {
    method: 'GET',
    headers: {
      "Content-Type": "application/json",
    },
  }
  return fetch(url, {
    ...requestInit,
    ...defaultRequestInit
  })
}

export async function post(url: string, requestInit?: RequestInit) {
  const defaultRequestInit: RequestInit = {
    method: 'POST',
    headers: {
      "Content-Type": "application/json",
    },
  }
  return fetch(url, {
    ...requestInit,
    ...defaultRequestInit
  })
}
