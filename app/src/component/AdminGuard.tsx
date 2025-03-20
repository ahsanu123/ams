import { useNavigate } from 'react-router'
import './AdminGuard.css'
import VirtualKeypad from './VirtualKeypad'

// TODO: make good logic for this 
// admin guard component, 
// make sure to use hashing to store password 
// and use cookie to save key for some duration

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
