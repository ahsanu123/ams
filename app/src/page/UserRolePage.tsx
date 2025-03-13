import { RoleManagerService, type IdentityRoleIntKey } from "../api/generated"
import { authorize, UserClaimTypes, type AuthorizationModel } from "../model/authorization-model.ts.bak"
import type { Route } from "./+types/UserRolePage"
import { defaultClient } from "../api/constant"
import { useState } from "react"

const authorization: AuthorizationModel = {
  claims: [
    {
      type: UserClaimTypes.role,
      value: "SUPERADMIN"
    }
  ]
}

interface AddRoleError {
  succeeded: boolean
  errors: {
    code: string
    description: string
  }[]
}

interface ClientLoaderType {
  role: string[]
  availableRole: IdentityRoleIntKey[]
}

export async function clientLoader({ params }: Route.ClientLoaderArgs) {
  const role = await RoleManagerService.getRoleManagerGetRoleForUserByUserName({
    client: defaultClient,
    path: {
      userName: params.userName
    }
  })

  const availableRole = await RoleManagerService.getRoleManagerGetRoles({ client: defaultClient })

  const authResult = await authorize(authorization)
  const data = role.data as string[]

  const returnData: ClientLoaderType = {
    role: data,
    availableRole: availableRole.data as IdentityRoleIntKey[]
  }

  return authResult === undefined ? returnData : authResult

}

export default function UserRolePage({
  loaderData,
  params
}: Route.ComponentProps) {


  const [selectedRole, setSelectedRole] = useState<string | undefined>(undefined)
  const [roleErrorMessage, setRoleErrorMessage] = useState<AddRoleError>()

  const roles = (loaderData as ClientLoaderType).role
  const availableRole = (loaderData as ClientLoaderType).availableRole

  const handleOnAddOrRemoveRole = async (remove: boolean) => {

    const result = (remove
      ? await RoleManagerService.postRoleManagerRemoveRoleForUserByUserName({
        client: defaultClient,
        path: {
          userName: params.userName
        },
        body: selectedRole
      })
      : await RoleManagerService.postRoleManagerAddRoleForUserByUserName({
        client: defaultClient,
        path: {
          userName: params.userName
        },
        body: selectedRole
      })
    )

    const data = result.data as AddRoleError

    if (!data.succeeded) setRoleErrorMessage(data)

    if (data.succeeded) setRoleErrorMessage(undefined)

  }

  const renderAddOrRemoveRole = (remove: boolean) => (
    <>
      <hr />

      <b>
        {remove ? "Remove " : "Add "}
        Role For {params.userName}
      </b>
      <sub>(need to re-login to work)</sub>
      <br />

      <select
        onChange={(event) => setSelectedRole(event.target.value)}
      >
        <option />
        {availableRole.map((role) => (
          <option
            value={role.normalizedName!}
          >
            {role.name}
          </option>
        ))}
      </select>
      {" "}

      <button
        disabled={selectedRole === undefined}
        onClick={() => handleOnAddOrRemoveRole(remove)}
      >
        {remove ? "Remove " : "Add "}
        Role
      </button>
    </>
  )

  return (
    <>
      <h2>Role For {params.userName}</h2>
      {roles.map((role, index) => (
        <li
          key={`${role}-${index}`}
        >
          {role}
        </li>
      ))}

      {renderAddOrRemoveRole(false)}
      {renderAddOrRemoveRole(true)}

      {
        roleErrorMessage && roleErrorMessage.errors.map((item, index) => (
          <li
            style={{
              color: 'red'
            }}
            key={`error-${index}`}
          >
            ({item.code}) {" - "}
            {item.description}
          </li>
        ))
      }
    </>
  )
}
