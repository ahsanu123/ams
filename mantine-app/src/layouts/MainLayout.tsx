import BottomInformation from "@/components/BottomInformation";
import BottomCornerLeftInfo from "@/components/BottomLeftInformation";
import RightSideBar from "@/components/RightSideBar";
import { useLayoutStore } from "@/utilities/layout-store";
import { Box } from "@mantine/core"
import { useElementSize, useViewportSize } from "@mantine/hooks"
import { useEffect } from "react";
import { Outlet } from "react-router"

export default function MainLayout() {
  const ld = useViewportSize();

  const { ref: mainRef, width: mainWidth, height: mainHeight } = useElementSize();
  const { ref: sideBarRef, width: sideBarWidth, height: sideBarHeight } = useElementSize();
  const { ref: bottomInfoRef, width: bottomInfoWidth, height: bottomInfoHeight } = useElementSize();
  const { ref: cornerBottomInfoRef, width: cornerBottomInfoWidth, height: cornerBottomInfoHeight } = useElementSize();

  const isMeasured =
    mainWidth > 0 &&
    mainHeight > 0 &&
    sideBarWidth > 0 &&
    sideBarHeight > 0 &&
    bottomInfoWidth > 0 &&
    bottomInfoHeight > 0 &&
    cornerBottomInfoWidth > 0 &&
    cornerBottomInfoHeight > 0

  const setAll = useLayoutStore(store => store.setAll);
  const setIsReady = useLayoutStore(store => store.setIsReady);

  useEffect(() => {
    setAll({
      mainWidth,
      mainHeight,
      sideBarWidth,
      sideBarHeight,
      bottomInfoWidth,
      bottomInfoHeight,
      cornerBottomInfoWidth,
      cornerBottomInfoHeight,
    })
    setIsReady(isMeasured);
  }, [
    mainWidth,
    mainHeight,
    sideBarWidth,
    sideBarHeight,
    bottomInfoWidth,
    bottomInfoHeight,
    cornerBottomInfoWidth,
    cornerBottomInfoHeight
  ])

  return (
    <Box
      style={{
        backgroundColor: 'gray',
        padding: "5px",
        height: `${ld.height}px`,
        display: 'grid',
        gridTemplateColumns: "repeat(12, 1fr)",
        gridTemplateRows: "repeat(12, 1fr)",
        gap: "5px"
      }}
    >
      <Box
        ref={mainRef}
        bg={'white'}
        style={{
          gridColumn: "1/10",
          gridRow: "1/12",
          borderRadius: "5px"
        }}
      >
        <Outlet />
      </Box>
      <Box
        ref={sideBarRef}
        bg={'white'}
        style={{
          gridColumn: "10/13",
          gridRow: "1/12",
          borderRadius: "5px"
        }}
      >
        <RightSideBar />
      </Box>
      <Box
        ref={bottomInfoRef}
        bg={'white'}
        style={{
          alignContent: "center",
          gridColumn: "1/10",
          gridRow: "12/13",
          borderRadius: "5px"
        }}
      >
        <BottomInformation />
      </Box>
      <Box
        ref={cornerBottomInfoRef}
        bg={'white'}
        style={{
          alignContent: "center",
          gridColumn: "10/13",
          gridRow: "12/13",
          borderRadius: "5px"
        }}
      >
        <BottomCornerLeftInfo />
      </Box>
    </Box>
  )
}
