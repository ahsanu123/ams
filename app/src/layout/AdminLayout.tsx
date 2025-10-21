import { AppRoutes } from "@/routes";
import { useMainLayoutStore } from "@/state";
import { Outlet, useNavigate } from "react-router";
import Clock from "../component/Clock";
import amsLogo from "../svg/ams-icon.svg";
import { useAdminPageStore } from "@/page";
import { useEffect } from "react";
import "./AdminLayout.css";
import { Button, Flex, Stack, Image, Heading, Box, Text, Breadcrumb } from "@chakra-ui/react";
import TreeMenuComponent from "@/page/admin-page/components/TreeMenu";

export default function AdminLayout() {
  const navigate = useNavigate();
  const headerInformation = useMainLayoutStore(state => state.headerInformation)
  const menuPath = useAdminPageStore(state => state.menuPath)
  const menuPathId = useAdminPageStore(state => state.menuPathId)

  const handleOnBackToCustomerTakingPage = () => {
    navigate(`${AppRoutes.Root}`)
  }

  useEffect(() => {
    if (menuPathId !== undefined) {
      navigate(menuPathId)
    }
  }, [menuPath])

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

          <Breadcrumb.Root>
            <Breadcrumb.List>
              <Breadcrumb.Item>
                🏠
              </Breadcrumb.Item>
              {menuPath.split('/').map((path) =>
                <>
                  <Breadcrumb.Item>
                    {path.replaceAll('-', ' ')}
                  </Breadcrumb.Item>
                  <Breadcrumb.Separator />
                </>
              )}
            </Breadcrumb.List>
          </Breadcrumb.Root>
        </Stack>

        <Box>
          <Heading>{headerInformation.title}</Heading>
          <sub>{headerInformation.description}</sub>
        </Box>
      </Flex>
      <hr />

      <Flex
        className="menu-and-outlet"
      >
        <Box flex={1}>
          <TreeMenuComponent />
        </Box>
        <Box flex={5}>
          <Outlet />
        </Box>
      </Flex>

      <footer>
        <Flex alignItems={'center'}>
          <Button
            variant={'ghost'}
            onClick={() => handleOnBackToCustomerTakingPage()}
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
