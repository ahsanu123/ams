import Cookies from "js-cookie";

type UserRoleType = 'Admin' | 'Buyer'

export interface AuthenticationCookieData {
  username: string,
  isAuthenticated: boolean,
  role: UserRoleType
}

export type CookieKey = 'authentication-session'

export function getCookie<T>(key: CookieKey) {
  const cookie = Cookies.get(key);
  if (!cookie) return null;

  try {
    const parsedCookie = JSON.parse(cookie) as T;
    return parsedCookie
  } catch (error) {
    console.log("error occur when parsing cookie", error)
  }
}

export function setCookie(key: CookieKey, object: Object) {
  Cookies.set(key, JSON.stringify(object), { expires: 1 })
}

export function removeCookie(key: CookieKey) {
  Cookies.remove(key)
}
