import { RoleManagerService, UserManagerService, type IdentityRoleIntKey, type IdentityUserIntKey } from "../api/generated"
import { authorize, UserClaimTypes, type AuthorizationModel } from "../model/authorization-model"
import { useEffect, useState } from "react"
import { defaultClient } from "../api/constant"
import { useNavigate } from "react-router"
import { AppRoutes } from "../routes"

const MAX_USER_TABLE_HEAD = 4
const MAX_ROLE_TABLE_HEAD = 3

const authorization: AuthorizationModel = {
  claims: [
    {
      type: UserClaimTypes.role,
      value: "SUPERADMIN"
    }
  ]
}

export async function clientLoader() {
  return authorize(authorization)
}

export default function SuperAdminPage() {

  const navigate = useNavigate()

  const [users, setUsers] = useState<IdentityUserIntKey[]>()
  const [roles, setRoles] = useState<IdentityRoleIntKey[]>()

  const getRoles = async () => {
    const roles = await RoleManagerService.getRoleManagerGetRoles({ client: defaultClient })
    setRoles(roles.data as IdentityRoleIntKey[])
  }

  const getUsers = async () => {
    const users = await UserManagerService.getUserManagerListUsers({ client: defaultClient })
    setUsers(users.data as IdentityUserIntKey[])
  }

  const handleOnShowRole = (userName: string) => {
    navigate(`/${AppRoutes.PagePrefix}${AppRoutes.UserRolePage}/${userName}`)
  }

  const renderTableHead = (user: (IdentityUserIntKey | IdentityRoleIntKey), isRoleType: boolean = false) => {

    let keyType: keyof IdentityUserIntKey

    const keyList: string[] = []

    for (keyType in user) {
      keyList.push(keyType)
    }

    return (
      <tr>
        {keyList.slice(0, isRoleType ? MAX_ROLE_TABLE_HEAD : MAX_USER_TABLE_HEAD).map((key) => (
          <td><b>{key}</b></td>
        ))}
      </tr>
    )
  }

  const renderTableRow = (user: IdentityUserIntKey, isRoleType: boolean = false) => {

    let t: keyof IdentityUserIntKey
    const data: string[] = []
    for (t in user) {
      data.push(user[t] as string)
    }
    return (
      <tr>
        {data.slice(0, isRoleType ? MAX_ROLE_TABLE_HEAD : MAX_USER_TABLE_HEAD).map((item) => (
          <td
            align="left"
          >
            {item}
          </td>
        ))}

        {
          !isRoleType && (
            <td>
              <button
                onClick={() => handleOnShowRole(user.userName ?? "")}
              >
                show roles
              </button>
            </td>
          )
        }

      </tr>
    )
  }

  const renderTable = (users: (IdentityUserIntKey | IdentityRoleIntKey)[], isRoleType: boolean = false) => (
    <table>
      <thead>
        {renderTableHead(users[0], isRoleType)}
      </thead>

      <tbody>
        {users.map((user) => renderTableRow(user, isRoleType))}
      </tbody>

    </table>
  )

  useEffect(() => {
    getUsers()
    getRoles()
  }, [])

  return (
    <>
      <h1>Super Admin Page</h1>

      <hr />

      <h3>👮 List User</h3>
      {users && renderTable(users)}
      <hr />

      <h3>👯 List Role</h3>
      {roles && renderTable(roles, true)}
      <hr />

    </>
  )
}
