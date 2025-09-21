import Calendar from '@/component/Calendar';
import type { HeaderInformation } from '@/model';
import { AppRoutes } from '@/routes';
import { useAdminPageStore, useMainLayoutStore } from '@/state';
import { calculatePassword } from '@/utility';
import { useEffect } from 'react';
import { useNavigate } from 'react-router';
import VirtualKeypad from '../component/VirtualKeypad';
import './AdminGuardPage.css';

export default function AdminGuardComponent() {
  const navigate = useNavigate();

  const loggedAdminInformationMessage: HeaderInformation = {
    title: 'Welcome Admin',
    description: 'you now able to do admin operation'
  }
  const notLoggedAdminInformationMessage: HeaderInformation = {
    title: 'Admin Only',
    description: 'you must login!'
  }

  const loginMessage = useAdminPageStore(state => state.loginMessage)
  const setLoginMessage = useAdminPageStore(state => state.setLoginMessage)

  const isAdmin = useMainLayoutStore(state => state.isAdmin)
  const setIsAdmin = useMainLayoutStore(state => state.setIsAdmin)
  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)
  const setPage = useMainLayoutStore(state => state.setPage)

  const handleOnConfirmPassword = (value: number) => {
    const password = calculatePassword()

    if (value == password) {
      setHeaderInformation(loggedAdminInformationMessage)
      setLoginMessage('')
      setIsAdmin(true)
    }
    else setLoginMessage("Wrong Broo!!!")
  }

  const handleOnLogOut = () => {
    setLoginMessage('')
    setHeaderInformation(notLoggedAdminInformationMessage)
    setIsAdmin(false)
  }

  const handleOnClickBack = () => {
    setPage(AppRoutes.PagePrefix)
    navigate(`${AppRoutes.PagePrefix}`)
  }

  const ListAdminMenu = () => (
    <>
      <div>
        <h2>Welcome</h2>

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
    setHeaderInformation(notLoggedAdminInformationMessage)
  }, [])
  return (
    <div
      className='admin-guard'
    >
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
    </div>
  )
}
