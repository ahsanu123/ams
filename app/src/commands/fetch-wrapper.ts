export async function fetchWithDateTimeCompitable(url: string, requestInit: RequestInit, body?: object) {
  let transformedBody: string | undefined

  if (body !== null)
    transformedBody = JSON.stringify(transformObjectDates(body));

  if (requestInit.method == 'GET')
    return fetch(url, { ...requestInit });

  return fetch(url, { ...requestInit, body: transformedBody });
}

function transformObjectDates(obj: any) {
  const transformed = { ...obj };

  for (const key in transformed) {
    if (transformed[key] instanceof Date) {
      transformed[key] = transformed[key].toISOString().replace('Z', '');
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
