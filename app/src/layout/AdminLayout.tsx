import { useAdminPageStore } from "@/page";
import TreeMenuComponent from "@/page/admin-page/components/TreeMenu";
import { AdminRoutes, AppRoutes } from "@/routes";
import { useMainLayoutStore } from "@/state";
import { Box, Breadcrumb, Button, Flex, Heading, Image, Stack, Text } from "@chakra-ui/react";
import { useEffect } from "react";
import { Outlet, useNavigate } from "react-router";
import Clock from "../component/Clock";
import amsLogo from "../svg/ams-icon.svg";
import "./AdminLayout.css";
import { AiFillCopyrightCircle } from "react-icons/ai";
import React from "react";

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
        width={'100%'}
        padding={'0 20px'}
        backgroundImage={'url(/ams-hero-4.png)'}
        backgroundSize={'contain'}
        height={'120px'}
        backgroundColor={'#becda8'}
        className="logo-and-info">

        <Stack className="logo-and-breadcrumbs">
          <Stack direction={'row'} alignItems={'center'}>
            <Button
              variant={'ghost'}
              onClick={() => navigate(`${AdminRoutes.AdminRoot}`)}
            >
              <Image
                src={amsLogo}
                height={30}
                width={35}
              />
              <Heading size={"2xl"}>
                AMS
              </Heading>
            </Button>

            <Clock />

            <Breadcrumb.Root>
              <Breadcrumb.List>
                <Breadcrumb.Item>
                  🏠
                </Breadcrumb.Item>
                {menuPath.split('/').map((path) =>
                  <React.Fragment key={path}>
                    <Breadcrumb.Item>
                      {path.replaceAll('-', ' ')}
                    </Breadcrumb.Item>
                    <Breadcrumb.Separator />
                  </React.Fragment>
                )}
              </Breadcrumb.List>
            </Breadcrumb.Root>

          </Stack>
          <Flex justifyContent={'start'} gap={2} alignItems={'center'}>
            <Heading>{headerInformation.title}</Heading>
            <Text>{headerInformation.description}</Text>
          </Flex>
        </Stack>

      </Flex>
      <hr />

      <Flex
        className="menu-and-outlet"
      >
        <Box width={'280px'}>
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
