import { createClient } from "@hey-api/client-fetch"

export const BASE_URL = "http://localhost:5136"
export const defaultClient = createClient({ baseUrl: BASE_URL, credentials: 'include', mode: 'cors' })

// TODO: fix return url parameter
export const externalAuthenticationUrlBuilder = (provider: ExternalLoginProvider) => (
  `${BASE_URL}/Account/ExternalLogin?provider=${provider}&returnUrl=%2F`
)

export enum ExternalLoginProvider {
  Google = "Google",
  Github = "Github"
}

export enum AmpasSummaryDurationEnum {
  Daily,
  Monthly
}
