import { render, screen } from '@testing-library/react'
import { describe, expect, it, beforeEach } from 'vitest'
import { ProductList } from '@/component/ProductList'
import { useProductStore } from '@/state'

describe("Product List Component Test", () => {

  beforeEach(() => useProductStore.setState({
    products: []
  }))

  it('product_list_no_product', () => {
    render(<ProductList />)
    screen.debug()
    expect(screen.getByText("No Product Available")).toBeInTheDocument()
  })

  it('product_list_with_product', () => {
    useProductStore.setState({
      products: [
        {
          id: 0,
          userId: 0,
          paid: false,
          productionDate: new Date(),
          takenDate: new Date(),
          price: 0,
          amount: 13,
          description: 'new product added'
        }
      ],
    });

    render(<ProductList />);
    screen.debug()
    expect(screen.getByText('take Product: 13')).toBeInTheDocument();

  });

})
