import { LOGGED_ADMIN_INFORMATION_MESSAGE, NOT_LOGGED_ADMIN_INFORMATION_MESSAGE } from '@/constants';
import { AppRoutes } from '@/routes';
import { useAdminPageStore, useMainLayoutStore } from '@/state';
import { calculatePassword } from '@/utility';
import { useEffect } from 'react';
import { useNavigate } from 'react-router';
import VirtualKeypad from '../component/VirtualKeypad';
import './AdminGuardPage.css';
import MenuTreeComponent from '@/component/MenuTree';

export default function AdminGuardComponent() {
  const navigate = useNavigate();

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
        {
          isAdmin ? <MenuTreeComponent />
            : (
              <div>
                <VirtualKeypad
                  inputType='number'
                  handleOnConfirm={handleOnConfirmPassword}
                />
              </div>
            )
        }
        <div>
          {activeMenu?.component()}
        </div>
      </div>

      <button
        onClick={handleOnClickBack}
      >
        <b>🍚 Back</b>
      </button>

      {
        isAdmin && (
          <>
            {" "}
            <button
              onClick={() => handleOnLogOut()}
            >
              <b>Log Out</b>

            </button>
          </>
        )
      }

    </div>
  )
}
