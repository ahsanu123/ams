import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import type { Product, User, HeaderInformation } from 'model'
import { AppRoutes } from '@/routes'
import { getCookie, type AuthenticationCookieData, type ICalendarCell } from '@/utility'
import { generateMockProduct } from '@/mock'

interface MainLayoutStore {
  products: Array<Product>,
  addProduct: (product: Product) => boolean,
  updateProduct: (id: number, productData: Partial<Product>) => boolean,
  deleteProduct: (id: number) => boolean,
  setProducts: (products: Array<Product>) => void,

  selectedMonth: Date,
  setSelectedDate: (date: Date) => void,

  user?: User,
  setUser: (user: User | undefined) => void,

  lastSelectedUser?: User,
  setLastSelectedUser: (user: User | undefined) => void,

  currentPage: AppRoutes,
  pageRoutes: string[],
  setPage: (pageRoute: AppRoutes) => void,
  nextPage: () => void,
  prevPage: () => void,

  isAuthenticationCookieExist: boolean,
  checkIsAuthenticationCookieExist: () => void,

  listUser: User[],
  setListUser: (users: User[]) => void,

  productPrice: number,
  setProductPrice: (price: number) => void

  allProductOfThisMonth: Array<Product>,
  setAllProductOfThisMonth: (products: Array<Product>) => void

  headerInformation: HeaderInformation,
  setHeaderInformation: (information: HeaderInformation) => void

  isAdmin: boolean,
  setIsAdmin: (admin: boolean) => void

  calendarCells: ICalendarCell[],
  setCalendarCells: (calendarCells: ICalendarCell[]) => void;
}

export const useMainLayoutStore = create<MainLayoutStore>()(
  immer((set) => ({

    // mocked
    products: generateMockProduct(),
    addProduct: (product) => {
      set((state) => {
        state.products.push(product)
      })
      return true
    },

    updateProduct: (id, productData) => {
      let success: boolean = false;

      set((state) => {
        const product = state.products.find((item) => item.id === id)
        if (product) {
          Object.assign(product, productData)
          success = true
        }
      })

      return success
    },

    deleteProduct: (id) => {
      let success: boolean = false

      set((state) => {
        const productIndex = state.products.findIndex((item) => item.id === id)

        if (productIndex !== -1) {
          state.products = state.products.filter((product) => product.id !== id)
          success = true
        }
      })

      return success
    },

    setProducts: (products) => {
      set((state) => {
        state.products = products
      })
    },

    selectedMonth: new Date(),
    setSelectedDate: (date) => {
      set((state) => {
        state.selectedMonth = date
      })
    },


    setUser: (user) => {
      set((state) => {
        state.user = user
      })
    },

    currentPage: AppRoutes.PagePrefix,
    pageRoutes: Object.values(AppRoutes),

    setPage: (pageRoute) => {
      set((state) => {
        state.currentPage = pageRoute
      })
    },

    nextPage: () => {
      set((state) => {
        const currentPageIndex = state.pageRoutes.findIndex((item) => item === state.currentPage)
        if (currentPageIndex + 1 < state.pageRoutes.length)
          state.currentPage = state.pageRoutes[currentPageIndex + 1] as AppRoutes
        else
          state.currentPage = state.pageRoutes[0] as AppRoutes
      })
    },

    prevPage: () => {
      set((state) => {
        const currentPageIndex = state.pageRoutes.findIndex((item) => item === state.currentPage)
        if (currentPageIndex > 0)
          state.currentPage = state.pageRoutes[currentPageIndex - 1] as AppRoutes
        else
          state.currentPage = state.pageRoutes[state.pageRoutes.length - 1] as AppRoutes
      })
    },

    isAuthenticationCookieExist: false,
    checkIsAuthenticationCookieExist: () => {
      set((state) => {
        state.isAuthenticationCookieExist = !!getCookie<AuthenticationCookieData>('authentication-session')
      })
    },

    listUser: [],
    setListUser: (users) => {
      set((state) => {
        state.listUser = users
      })
    },

    productPrice: 0,
    setProductPrice: (price) => {
      set((state) => {
        state.productPrice = price
      })
    },

    allProductOfThisMonth: [],
    setAllProductOfThisMonth: (products) => {
      set((state) => {
        state.allProductOfThisMonth = products
      })
    },

    headerInformation: {
      title: '',
      description: ''
    },

    setHeaderInformation: (information) => {
      set((state) => {
        state.headerInformation = information
      })
    },

    isAdmin: false,
    setIsAdmin: (admin) => {
      set((state) => {
        state.isAdmin = admin
      })
    },

    calendarCells: [],
    setCalendarCells: (calendarCells) => {
      set((state) => {
        state.calendarCells = calendarCells
      })
    },

    lastSelectedUser: undefined,
    setLastSelectedUser: (user) => {
      set((state) => {
        state.lastSelectedUser = user
      })
    }

  }))
)
