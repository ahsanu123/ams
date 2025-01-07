import { useEffect } from "react"
import CampaignCardManager from "../component/CampaignCardManager"
import { authorize, UserClaimTypes, type AuthorizationModel, type UserClaims } from "../model/authorization-model"

const authorization: AuthorizationModel = {
  claims: [
    {
      type: UserClaimTypes.role,
      value: "GENERALADMIN"
    },
    {
      type: UserClaimTypes.role,
      value: "ASIAADMIN"
    },
    {
      type: UserClaimTypes.role,
      value: "EUROPEADMIN"
    },
    {
      type: UserClaimTypes.role,
      value: "AMERICANADMIN"
    },
  ]
}

export async function clientLoader() {
  return authorize(authorization)
}

export default function CampaignManagerPage() {

  return (
    <>
      <h2>Campaign Manager</h2>
      <CampaignCardManager />
    </>
  )
}
