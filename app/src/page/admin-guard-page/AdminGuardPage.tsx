import { LOGGED_ADMIN_INFORMATION_MESSAGE, NOT_LOGGED_ADMIN_INFORMATION_MESSAGE } from '@/constants';
import { AdminRoutes, AppRoutes } from '@/routes';
import { useMainLayoutStore } from '@/state';
import { calculatePassword } from '@/utility';
import { useEffect } from 'react';
import { useNavigate } from 'react-router';
import VirtualKeypad from '../../component/VirtualKeypad';
import './AdminGuardPage.css';


export default function AdminGuardComponent() {
  const navigate = useNavigate();

  const setIsAdmin = useMainLayoutStore(state => state.setIsAdmin)
  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)

  const handleOnConfirmPassword = (value: number) => {
    const password = calculatePassword()

    if (value === password) {
      setHeaderInformation(LOGGED_ADMIN_INFORMATION_MESSAGE)
      setIsAdmin(true)
      navigate(`${AdminRoutes.AdminRoot}`)
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

  const handleOnBackButtonClicked = () => navigate(`${AppRoutes.Root}`)

  useEffect(() => {
    setHeaderInformation(NOT_LOGGED_ADMIN_INFORMATION_MESSAGE)
  }, [])


  return (
    <div
      className='admin-guard'
    >
      <button
        onClick={() => handleOnBackButtonClicked()}
      >
        Back
      </button>
      <div className='main-container'>
        <div>
          <VirtualKeypad
            inputType='number'
            handleOnConfirm={handleOnConfirmPassword}
          />
        </div>
      </div>

    </div>
  )
}
