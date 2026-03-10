import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'

interface LayoutState {
  mainWidth: number,
  mainHeight: number,

  sideBarWidth: number,
  sideBarHeight: number,

  bottomInfoWidth: number,
  bottomInfoHeight: number,

  cornerBottomInfoWidth: number,
  cornerBottomInfoHeight: number,
  isReady: boolean
}

interface LayoutStore extends LayoutState {
  getAll: () => Omit<LayoutState, 'isReady'>,

  setAll: (args: {
    mainWidth: number,
    mainHeight: number,

    sideBarWidth: number,
    sideBarHeight: number,

    bottomInfoWidth: number,
    bottomInfoHeight: number,

    cornerBottomInfoWidth: number,
    cornerBottomInfoHeight: number
  }) => void,

  setMain: (width: number, height: number) => void,
  setSideBar: (width: number, height: number) => void,
  setBottomInfo: (width: number, height: number) => void,
  setCornerBottomInfo: (width: number, height: number) => void,

  setIsReady: (ready: boolean) => void,
}

const layoutState: LayoutState = {
  mainWidth: 0,
  mainHeight: 0,

  sideBarWidth: 0,
  sideBarHeight: 0,

  bottomInfoWidth: 0,
  bottomInfoHeight: 0,

  cornerBottomInfoWidth: 0,
  cornerBottomInfoHeight: 0,

  isReady: false
}


export const useLayoutStore = create<LayoutStore>()(
  immer((set, get) => ({
    ...layoutState,

    setMain: (width: number, height: number) => {
      set((state) => {
        state.mainWidth = width;
        state.mainHeight = height;
      })
    },

    setSideBar: (width: number, height: number) => {
      set((state) => {
        state.sideBarWidth = width;
        state.sideBarHeight = height;
      })
    },

    setBottomInfo: (width: number, height: number) => {
      set((state) => {
        state.bottomInfoWidth = width;
        state.bottomInfoHeight = height;
      })
    },

    setCornerBottomInfo: (width: number, height: number) => {
      set((state) => {
        state.cornerBottomInfoWidth = width;
        state.cornerBottomInfoHeight = height;
      })
    },

    getAll: () => {
      const state = get()
      return {
        mainWidth: state.mainWidth,
        mainHeight: state.mainHeight,

        sideBarWidth: state.sideBarWidth,
        sideBarHeight: state.sideBarHeight,

        bottomInfoWidth: state.bottomInfoWidth,
        bottomInfoHeight: state.bottomInfoHeight,

        cornerBottomInfoWidth: state.cornerBottomInfoWidth,
        cornerBottomInfoHeight: state.cornerBottomInfoHeight,
      }
    },

    setAll: (args: {
      mainWidth: number,
      mainHeight: number,
      sideBarWidth: number,
      sideBarHeight: number,
      bottomInfoWidth: number,
      bottomInfoHeight: number,
      cornerBottomInfoWidth: number,
      cornerBottomInfoHeight: number
    }) => {
      set((state) => {
        state.mainWidth = args.mainWidth;
        state.mainHeight = args.mainHeight;

        state.sideBarWidth = args.sideBarWidth;
        state.sideBarHeight = args.sideBarHeight;

        state.bottomInfoWidth = args.bottomInfoWidth;
        state.bottomInfoHeight = args.bottomInfoHeight;

        state.cornerBottomInfoWidth = args.cornerBottomInfoWidth;
        state.cornerBottomInfoHeight = args.cornerBottomInfoHeight;
      })
    },

    setIsReady: (ready: boolean) => {
      set((state) => {
        state.isReady = ready;
      })
    },

  }))
)
