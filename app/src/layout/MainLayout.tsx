import { SecretRoutes } from "@/routes";
import { useMainLayoutStore } from "@/state";
import { Box, Breadcrumb, Button, Flex, Heading, Image, Stack, Text } from "@chakra-ui/react";
import { Outlet, useNavigate } from "react-router";
import Clock from "../component/Clock";
import amsLogo from "../svg/ams-icon.svg";
import "./AdminLayout.css";
import { AiFillCopyrightCircle } from "react-icons/ai";

export default function AdminLayout() {
  const navigate = useNavigate();
  const headerInformation = useMainLayoutStore(state => state.headerInformation)

  const handleAdminLogin = () => {
    navigate(`${SecretRoutes.AdminGuardPage}`)
  }

  return (
    <Box className="admin-header">
      <Flex
        width={'100%'}
        padding={'0 20px'}
        backgroundImage={'url(/ams-hero-4.png)'}
        backgroundSize={'contain'}
        height={'120px'}
        backgroundColor={'#becda8'}
        className="logo-and-info">

        <Stack className="logo-and-breadcrumbs">
          <Stack direction={'row'} alignItems={'center'}>
            <Image
              src={amsLogo}
              height={30}
              width={35}
            />
            <Heading size={"2xl"} marginLeft={'10px'}>
              AMS
            </Heading>

            <Clock />

            <Heading>{headerInformation.title}</Heading>
            <Text>{headerInformation.description}</Text>
          </Stack>
        </Stack>
      </Flex>
      <hr />

      <Box>
        <Outlet />
      </Box>

      <footer>
        <Flex alignItems={'center'}>
          <Button
            variant={'ghost'}
            onClick={() => handleAdminLogin()}
          >
            <AiFillCopyrightCircle />
          </Button>
          <Text>
            Copyright {new Date().getFullYear()}
          </Text>
        </Flex>
      </footer>
    </Box>
  )
}
