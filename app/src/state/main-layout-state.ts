import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import type { ProductRecord, Product, User } from 'model'
import { AppRoutes } from '@/routes'
import { getCookie, type AuthenticationCookieData } from '@/utility'

interface MainLayoutStore {
  products: ProductRecord,
  addProduct: (product: Product) => boolean,
  updateProduct: (id: number, productData: Partial<Product>) => boolean,
  deleteProduct: (id: number) => boolean,

  selectedMonth: Date,
  setSelectedDate: (date: Date) => void,

  user?: User,
  setUser: (user: User) => void,

  currentPage: AppRoutes,
  pageRoutes: string[],
  setPage: (pageRoute: AppRoutes) => void,
  nextPage: () => void,
  prevPage: () => void,

  isAuthenticationCookieExist: boolean,
  checkIsAuthenticationCookieExist: () => void,

}

export const useMainLayoutStore = create<MainLayoutStore>()(
  immer((set) => ({

    products: [],
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
    }

  }))
)
