import { Customer } from "@/bindings/Customer";
import CustomerPicker from "@/components/CustomerPicker";
import VirtualKeypad from "@/components/VirtualKeypad";
import { useSidebarStore } from "@/global-stores/right-sidebar-store";
import { useEffect, useState } from "react";

export default function MainPageCustomerPicker() {

  const [selectedCustomer, setSelectedCustomer] = useState<Customer | undefined>(undefined)

  const setSidebarTitle = useSidebarStore(store => store.setTitle)

  const handleOnConfirm = (value: number) => {
    setSelectedCustomer(undefined)
  }

  useEffect(() => {
    setSidebarTitle(selectedCustomer ? "Masukan Jumlah" : "Pilih Nama")
  }, [selectedCustomer])

  const customers: Customer[] = [
    {
      customer_id: 0,
      customer_name: "Tresno",
      is_active: false,
      is_admin: false,
      created_date: new Date(),
      updated_date: new Date()
    },
    {
      customer_id: 0,
      customer_name: "Sinin",
      is_active: false,
      is_admin: false,
      created_date: new Date(),
      updated_date: new Date()
    },
    {
      customer_id: 0,
      customer_name: "Misbah",
      is_active: false,
      is_admin: false,
      created_date: new Date(),
      updated_date: new Date()
    },
    {
      customer_id: 0,
      customer_name: "Lurah",
      is_active: false,
      is_admin: false,
      created_date: new Date(),
      updated_date: new Date()
    },
  ]
  return (
    <>
      {selectedCustomer ? (
        <VirtualKeypad
          handleOnConfirm={handleOnConfirm}
        />
      )
        : (
          <CustomerPicker
            customers={customers}
            onSelectedCustomer={(customer) => setSelectedCustomer(customer)}
          />
        )}

    </>
  )
}
