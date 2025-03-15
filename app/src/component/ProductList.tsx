import { useProductStore } from "@/state"

export const ProductList = () => {

  const products = useProductStore((state) => state.products)

  if (products.length === 0)
    return <div>No Product Available</div>

  return (
    <ul>
      {products.map((product, index) => (
        <li
          key={index}
        >
          take Product: {product.amount}
        </li>
      ))}
    </ul>
  )

}
