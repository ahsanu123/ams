import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import type { ProductRecord, Product } from 'model'

interface ProductStore {
  products: ProductRecord
  addProduct: (product: Product) => boolean
  updateProduct: (id: number, productData: Partial<Product>) => boolean
  deleteProduct: (id: number) => boolean
}

export const useProductStore = create<ProductStore>()(
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
    }

  }))
)
