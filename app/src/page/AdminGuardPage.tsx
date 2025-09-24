import { LOGGED_ADMIN_INFORMATION_MESSAGE, NOT_LOGGED_ADMIN_INFORMATION_MESSAGE } from '@/constants';
import { AppRoutes } from '@/routes';
import { useAdminPageStore, useEditPageStore, useMainLayoutStore } from '@/state';
import { calculatePassword } from '@/utility';
import { useEffect } from 'react';
import { useNavigate } from 'react-router';
import VirtualKeypad from '../component/VirtualKeypad';
import './AdminGuardPage.css';
import { getUserCommand } from '@/commands';
import type { Route } from './+types/AdminGuardPage';

export async function clientLoader() {
  const userCommand = getUserCommand();
  const listUser = await userCommand.getUsers()

  return {
    listUser,
  }
}


export default function AdminGuardComponent({
  loaderData
}: Route.ComponentProps) {
  const navigate = useNavigate();

  const { listUser } = loaderData

  const isDialogvisible = useEditPageStore(state => state.isDialogVisible)

  const isAdmin = useMainLayoutStore(state => state.isAdmin)
  const setIsAdmin = useMainLayoutStore(state => state.setIsAdmin)
  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)
  const setPage = useMainLayoutStore(state => state.setPage)

  const activeMenu = useAdminPageStore(state => state.activeAdminMenu)

  const handleOnConfirmPassword = (value: number) => {
    const password = calculatePassword()

    if (value == password) {
      setHeaderInformation(LOGGED_ADMIN_INFORMATION_MESSAGE)
      setIsAdmin(true)
      navigate(`${AppRoutes.PagePrefix}${AppRoutes.MainAdminPage}`)
    }
    else setHeaderInformation({
      ...NOT_LOGGED_ADMIN_INFORMATION_MESSAGE,
      description: 'wrong password'
    })
  }

  const handleOnLogOut = () => {
    setHeaderInformation(NOT_LOGGED_ADMIN_INFORMATION_MESSAGE)
    setIsAdmin(false)
  }

  const handleOnClickBack = () => {
    handleOnLogOut()
    setPage(AppRoutes.PagePrefix)
    navigate(`${AppRoutes.PagePrefix}`)
  }

  useEffect(() => {
    setHeaderInformation(NOT_LOGGED_ADMIN_INFORMATION_MESSAGE)
  }, [])

  return (
    <div
      className='admin-guard'
    >
      <div className='main-container'>
        <div>
          <VirtualKeypad
            inputType='number'
            handleOnConfirm={handleOnConfirmPassword}
          />
        </div>
      </div>

      {/* <div> */}
      {/*   { */}
      {/*     !isDialogvisible && ( */}
      {/*       <> */}
      {/*         <button */}
      {/*           onClick={handleOnClickBack} */}
      {/*         > */}
      {/*           <b>🍚 Back</b> */}
      {/*         </button> */}
      {/**/}
      {/*         { */}
      {/*           isAdmin && ( */}
      {/*             <> */}
      {/*               {" "} */}
      {/*               <button */}
      {/*                 onClick={() => handleOnLogOut()} */}
      {/*               > */}
      {/*                 <b>Log Out</b> */}
      {/**/}
      {/*               </button> */}
      {/*             </> */}
      {/*           ) */}
      {/*         } */}
      {/*       </> */}
      {/*     ) */}
      {/*   } */}
      {/* </div> */}

    </div>
  )
}
