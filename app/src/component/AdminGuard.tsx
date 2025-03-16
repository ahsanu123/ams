import { useNavigate } from 'react-router'
import './AdminGuard.css'
import VirtualKeypad from './VirtualKeypad'

export default function AdminGuardComponent() {
  const navigate = useNavigate();
  const handleOnConfirmPassword = (value: number) => {
  }
  return (
    <div
      className='admin-guard'
    >
      <h2>Admin Only</h2>
      <VirtualKeypad
        inputType='number'
        handleOnConfirm={handleOnConfirmPassword}
      />
    </div>
  )
}
