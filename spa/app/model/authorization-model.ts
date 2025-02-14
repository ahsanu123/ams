import { UserInfoService, UserManagerService } from "../api/generated";
import { AppRoutes } from "../routes";
import { redirect } from "react-router";
import { defaultClient } from "../api/constant";
import type { RoleButtonKeys } from "./role-button-model";

export enum UserClaimTypes {
  name = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name",
  email = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress",
  role = "http://schemas.microsoft.com/ws/2008/06/identity/claims/role"

}

export interface UserClaim {
  type: UserClaimTypes,
  value: RoleButtonKeys,
}

export interface AuthorizationModel {
  claims: Array<UserClaim>;
}

export interface AuthenticationStatus {
  authenticated: boolean
}

export type UserClaims = Array<UserClaim>

export async function whoami() {
  const whoami = await UserInfoService.getUserInfoWhoAmI({ client: defaultClient })
  return whoami.data as UserClaims
}

export async function authorize(authSetting: AuthorizationModel) {

  const claims = await whoami()

  // user not logged in -> redirect to login page
  if (claims.length === 0) {
    return redirect(`/${AppRoutes.PagePrefix}${AppRoutes.SigninPage}`)
  }

  const authSatisfied = authSetting.claims.map((requiredClaim) =>
    claims.find((claim) =>
      (claim.type.includes(requiredClaim.type) && claim.value === requiredClaim.value)))


  // user logged in but not satisfied required role
  if (authSatisfied.every((auth) => auth === undefined)) {
    return redirect(`/${AppRoutes.PagePrefix}${AppRoutes.NotAllowedOrNotFound}`)
  }

  // else pass user to access it
}

