// FIXME: correct this

import { notifications } from "@mantine/notifications";
import { format, isValid, parseISO } from "date-fns";
import { throwDeprecation } from "node:process";

const DATE_FORMAT = "yyyy-MM-dd'T'HH:mm:ss"

// NOTE: Supports cases where `content-type` is other than `json`
const getBody = <T>(c: Response | Request): Promise<T> => {
  const contentType = c.headers.get('content-type');

  if (contentType && contentType.includes('application/json')) {
    return c.json();
  }

  if (contentType && contentType.includes('application/pdf')) {
    return c.blob() as Promise<T>;
  }

  return c.text() as Promise<T>;
};

// NOTE: Update just base url
const getUrl = (contextUrl: string): string => {
  const url = new URL(contextUrl);
  const pathname = url.pathname;
  const search = url.search;
  const baseUrl =
    process.env.NODE_ENV === 'production'
      ? 'productionBaseUrl'
      : 'http://localhost:3000';

  const requestUrl = new URL(`${baseUrl}${pathname}${search}`);

  return requestUrl.toString();
};

// FIXME: fix Authorization
const getHeaders = (headers?: HeadersInit): HeadersInit => {
  return {
    ...headers,
    Authorization: 'token',
    'dev': "1",
  };
};

export async function fetchWithDateTimeCompitable(url: string, requestInit: RequestInit) {
  let transformedBody: string | undefined

  if (requestInit.body !== undefined && requestInit.body !== null && typeof (requestInit.body) === 'string') {
    const reParseBody = JSON.parse(requestInit.body);
    transformedBody = JSON.stringify(transformObjectDates(reParseBody));
  }

  if (requestInit.method === 'GET')
    return fetch(url, { ...requestInit });

  else
    return fetch(url, { ...requestInit, body: transformedBody });
}

export function transformObjectDates(obj: any) {
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

export const fetchMutator = async <T>(
  url: string,
  options: RequestInit,
): Promise<T> => {
  const requestHeaders = getHeaders(options.headers);

  const requestInit: RequestInit = {
    ...options,
    headers: requestHeaders,
  };

  try {
    const response = await fetchWithDateTimeCompitable(url, requestInit);
    if (!response.ok) {
      const errText = await response.text();
      notifications.show({
        title: "Error",
        message: errText,
        autoClose: 1000,
        color: "red"
      });
      throw new Error(errText)
    }

    const parsedData = await getBody<T>(response);

    return parsedData as T;

  } catch (error) {
    console.error(error)
    return Promise.reject(error)
  }
};
