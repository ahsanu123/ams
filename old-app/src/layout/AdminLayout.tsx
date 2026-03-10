import { IS_INSIDE_TAURI } from "@/constants";
import { useAdminPageStore } from "@/page";
import TreeMenuComponent from "@/page/admin-page/components/TreeMenu";
import { AdminRoutes, AppRoutes } from "@/routes";
import { useMainLayoutStore } from "@/state";
import { Box, Breadcrumb, Button, CloseButton, Dialog, Flex, Heading, Image, Portal, Stack, Text } from "@chakra-ui/react";
import React, { useEffect } from "react";
import { AiFillCopyrightCircle, AiFillHeart, AiFillHome } from "react-icons/ai";
import { Outlet, useNavigate } from "react-router";
import Clock from "../component/Clock";
import amsLogo from "../svg/ams-icon.svg";
import "./AdminLayout.css";

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
        height={'140px'}
        backgroundColor={'#becda8'}
        className="logo-and-info">

        <Stack className="logo-and-breadcrumbs" gap={0}>
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
                  <AiFillHome color='brown' />
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

        <Flex
          className="copyright-container"
          alignItems={'center'}
          backgroundColor={'white'}>

          <Button
            size={'xl'}
            variant={'ghost'}
            onClick={() => handleOnBackToCustomerTakingPage()}
          >
            <AiFillCopyrightCircle />
          </Button>
          <Text fontSize={'xl'}>
            Copyright {new Date().getFullYear()}
          </Text>

          <Dialog.Root>
            <Dialog.Trigger asChild>
              <Button variant="ghost" size="lg">
                About
              </Button>
            </Dialog.Trigger>
            <Portal>
              <Dialog.Backdrop />
              <Dialog.Positioner>

                <Dialog.Content>

                  <Dialog.Header>
                    <Flex alignItems={'center'} gap={'10px'}>
                      <Image
                        src={amsLogo}
                        height={30}
                        width={35}
                      />
                      <Dialog.Title fontSize={'4xl'}>AMS - About</Dialog.Title>
                    </Flex>
                  </Dialog.Header>

                  <Dialog.Body>
                    <Text fontSize={'xl'}>
                      Ampas Management System (AMS) is a personal project designed to modernize and streamline the selling workflow of soybean dregs (ampas). It helps simplify daily operations by providing an organized, reliable system to record sales, track transactions, and manage day-to-day data with ease.
                    </Text>
                  </Dialog.Body>

                  <Dialog.Footer>
                    <Flex alignItems={'center'} gap={'10px'}>
                      Made With
                      <AiFillHeart size={'26px'} color='red' />
                      By Ah.
                      {' '}
                      {IS_INSIDE_TAURI ? "IS_INSIDE_TAURI" : "INSIDE_VITE_ENV"}
                    </Flex>
                  </Dialog.Footer>

                  <Dialog.CloseTrigger asChild>
                    <CloseButton size="sm" />
                  </Dialog.CloseTrigger>

                </Dialog.Content>

              </Dialog.Positioner>
            </Portal>
          </Dialog.Root>

        </Flex>

      </Flex>
      <hr />

      <Flex
        className="menu-and-outlet"
      >
        <Box minWidth={'280px'}>
          <TreeMenuComponent />
        </Box>
        <Box flex={5}>
          <Outlet />
        </Box>
      </Flex>

    </Box>
  )
}
