import { authorize, UserClaimTypes, type AuthorizationModel, type UserClaims } from "../model/authorization-model.ts.bak"

const authorization: AuthorizationModel = {
  claims: [
    {
      type: UserClaimTypes.role,
      value: "ASIAADMIN"
    }
  ]
}

export async function clientLoader() {
  return authorize(authorization)
}

export default function ClaimPage() {
  return (
    <>
      <h2>Claim Page</h2>

    </>
  )
}
