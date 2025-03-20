import { useNavigate } from 'react-router'
import './AdminGuardPage.css'
import VirtualKeypad from '../component/VirtualKeypad'
import { AppRoutes } from '@/routes';
import { getCookie, setCookie, type AuthenticationCookieData } from '@/utility';

// TODO: right now login is only handled for admin.

export default function AdminGuardComponent() {
  const navigate = useNavigate();

  const handleOnConfirmPassword = (value: number) => {
    // TODO: Replace this logic to request 
    // authentication from rust backend

    // const userData = command.loginAdmin();

    const authData: AuthenticationCookieData = {
      username: 'paijo',
      isAuthenticated: true,
      role: 'Admin'
    }
    setCookie('authentication-session', authData)
    navigate(`${AppRoutes.PagePrefix}`)
  }
  const handleOnClickBack = () => {
    navigate(`${AppRoutes.PagePrefix}`)
  }
  return (
    <div
      className='admin-guard'
    >
      <button
        onClick={handleOnClickBack}
      >
        <b>🍚 Back</b>
      </button>

      <h2>🍖 Admin Only</h2>
      <sub>
        you must login first
      </sub>
      <VirtualKeypad
        inputType='number'
        handleOnConfirm={handleOnConfirmPassword}
      />
    </div>
  )
}
