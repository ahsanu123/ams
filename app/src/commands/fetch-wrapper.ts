import { format, isValid, parseISO } from "date-fns";

const DATE_FORMAT = "yyyy-MM-dd'T'HH:mm:ss"

export async function fetchWithDateTimeCompitable(url: string, requestInit: RequestInit, body?: object) {
  let transformedBody: string | undefined

  if (body !== undefined)
    transformedBody = JSON.stringify(transformObjectDates(body));

  if (requestInit.method === 'GET')
    return fetch(url, { ...requestInit });

  else
    return fetch(url, { ...requestInit, body: transformedBody });
}

function transformObjectDates(obj: any) {
  const transformed = { ...obj };

  for (const key in transformed) {
    if (transformed[key] instanceof Date) {
      transformed[key] = format(transformed[key], DATE_FORMAT);
    }
    else if (typeof (transformed[key]) === "string") {
      const date = parseISO(transformed[key])
      const validDate = isValid(date)
      if (validDate)
        transformed[key] = format(date, DATE_FORMAT);
    } else if (typeof transformed[key] === 'object' && transformed[key] !== null) {
      transformed[key] = transformObjectDates(transformed[key]);
    }
  }

  return transformed;
}

export async function asJson<T>(response: Response): Promise<T> {
  const json = await response.json() as T
  return json
}

export async function asConstant<T>(response: Response): Promise<T> {
  const text = await response.text() as string
  const constant = JSON.parse(text) as T
  return constant
}

export async function get(url: string) {
  const defaultRequestInit: RequestInit = {
    method: 'GET',
    headers: {
      "Content-Type": "application/json",
    },
  }
  return fetchWithDateTimeCompitable(url, defaultRequestInit)
}

export async function post(url: string, body?: object) {
  const defaultRequestInit: RequestInit = {
    method: 'POST',
    headers: {
      "Content-Type": "application/json",
    },
  }
  return fetchWithDateTimeCompitable(url, defaultRequestInit, body)
}

export async function del(url: string, body?: object) {
  const defaultRequestInit: RequestInit = {
    method: 'DELETE',
    headers: {
      "Content-Type": "application/json",
    },
  }
  return fetchWithDateTimeCompitable(url, defaultRequestInit, body)
}
