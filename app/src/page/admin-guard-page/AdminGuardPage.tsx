import { LOGGED_ADMIN_INFORMATION_MESSAGE, NOT_LOGGED_ADMIN_INFORMATION_MESSAGE } from '@/constants';
import { AdminRoutes, AppRoutes } from '@/routes';
import { useMainLayoutStore } from '@/state';
import { calculatePassword, toaster } from '@/utility';
import { useEffect } from 'react';
import { useNavigate } from 'react-router';
import VirtualKeypad from '../../component/VirtualKeypad';
import './AdminGuardPage.css';
import { Box, Button, Stack } from '@chakra-ui/react';
import { AiOutlineArrowLeft } from 'react-icons/ai';


export default function AdminGuardComponent() {
  const navigate = useNavigate();

  const setIsAdmin = useMainLayoutStore(state => state.setIsAdmin)
  const setHeaderInformation = useMainLayoutStore(state => state.setHeaderInformation)

  const handleOnConfirmPassword = (value: number) => {
    const password = calculatePassword()

    if (value === password) {
      setHeaderInformation(LOGGED_ADMIN_INFORMATION_MESSAGE)
      setIsAdmin(true)
      toaster.create({
        title: 'Welcome',
        type: 'success'
      })
      navigate(`${AdminRoutes.AdminRoot}`)
    }
    else
      toaster.create({
        title: 'wrong password',
        type: 'error'
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
    <Box
      className='admin-guard'
    >
      <Stack className='main-container'>
        <Button
          maxWidth={'200px'}
          onClick={() => handleOnBackButtonClicked()}
        >
          <AiOutlineArrowLeft />
          Back
        </Button>
        <VirtualKeypad
          inputType='number'
          handleOnConfirm={handleOnConfirmPassword}
        />
      </Stack>

    </Box >
  )
}
