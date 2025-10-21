import { SecretRoutes } from "@/routes";
import { useMainLayoutStore } from "@/state";
import { Box, Button, Flex, Heading, Image, Stack, Text } from "@chakra-ui/react";
import { Outlet, useNavigate } from "react-router";
import Clock from "../component/Clock";
import amsLogo from "../svg/ams-icon.svg";
import "./AdminLayout.css";

export default function AdminLayout() {
  const navigate = useNavigate();
  const headerInformation = useMainLayoutStore(state => state.headerInformation)

  const handleAdminLogin = () => {
    navigate(`${SecretRoutes.AdminGuardPage}`)
  }

  return (
    <Box className="admin-header">
      <Flex
        className="logo-and-info">
        <Stack>
          <Flex>
            <Image
              src={amsLogo}
              height={25}
              width={26}
            />
            <Heading size={"2xl"}>
              AMS
            </Heading>
          </Flex>

          <Clock />

        </Stack>

        <Box>
          <Heading>{headerInformation.title}</Heading>
          <sub>{headerInformation.description}</sub>
        </Box>
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
            ©️
          </Button>
          <Text>
            Copyright {new Date().getFullYear()}
          </Text>
        </Flex>
      </footer>
    </Box>
  )
}
