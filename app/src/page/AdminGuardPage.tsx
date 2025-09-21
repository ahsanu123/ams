import Calendar from '@/component/Calendar';
import { LOGGED_ADMIN_INFORMATION_MESSAGE, NOT_LOGGED_ADMIN_INFORMATION_MESSAGE } from '@/constants';
import { AppRoutes } from '@/routes';
import { useAdminPageStore, useMainLayoutStore } from '@/state';
import { calculatePassword } from '@/utility';
import { useEffect } from 'react';
import { useNavigate } from 'react-router';
import VirtualKeypad from '../component/VirtualKeypad';
import './AdminGuardPage.css';

export default function AdminGuardComponent() {
  const navigate = useNavigate();

  const loginMessage = useAdminPageStore(state => state.loginMessage)
  const setLoginMessage = useAdminPageStore(state => state.setLoginMessage)

  const isAdmin = useMainLayoutStore(state => state.isAdmin)
  const setIsAdmin = useMainLayoutStore(state => state.setIsAdmin)
  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)
  const setPage = useMainLayoutStore(state => state.setPage)

  const handleOnConfirmPassword = (value: number) => {
    const password = calculatePassword()

    if (value == password) {
      setHeaderInformation(LOGGED_ADMIN_INFORMATION_MESSAGE)
      setLoginMessage('')
      setIsAdmin(true)
    }
    else setLoginMessage("Wrong Broo!!!")
  }

  const handleOnLogOut = () => {
    setLoginMessage('')
    setHeaderInformation(NOT_LOGGED_ADMIN_INFORMATION_MESSAGE)
    setIsAdmin(false)
  }

  const handleOnClickBack = () => {
    handleOnLogOut()
    setPage(AppRoutes.PagePrefix)
    navigate(`${AppRoutes.PagePrefix}`)
  }

  const ListAdminMenu = () => (
    <>
      <div>
        <div className='list-admin-menu'>
          <button>User Management</button>
          <button>Edit Record</button>
          <button>Payment Menu</button>
          <button>Edit Dreg Price</button>
          <button>Print Data</button>
        </div>
      </div>
    </>
  )

  useEffect(() => {
    setLoginMessage('')
    setHeaderInformation(NOT_LOGGED_ADMIN_INFORMATION_MESSAGE)
  }, [])
  return (
    <div
      className='admin-guard'
    >
      <div className='main-container'>
        {
          isAdmin ? ListAdminMenu() :
            (
              <div>
                <VirtualKeypad
                  inputType='number'
                  handleOnConfirm={handleOnConfirmPassword}
                />
              </div>
            )
        }
        <div>
          <Calendar />
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

      <sub>{loginMessage}</sub>

    </div>
  )
}
