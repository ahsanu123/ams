import { useGetAllCustomerByProps, usePostUpdateCustomer } from "@/api/v1/customer-management-controller/customer-management-controller";
import { Customer } from "@/api/v1/models";
import CustomerPicker from "@/components/CustomerPicker";
import VirtualKeypad from "@/components/VirtualKeypad";
import { useSidebarStore } from "@/global-stores/right-sidebar-store";
import { useEffect, useState } from "react";

export default function MainPageCustomerPicker() {

  const [selectedCustomer, setSelectedCustomer] = useState<Customer | undefined>(undefined)
  const setSidebarTitle = useSidebarStore(store => store.setTitle)

  const handleOnConfirm = (value: number) => {
    mutator.mutate({
      data: {
        customer_id: 0,
        customer_name: "llll",
        is_active: false,
        is_admin: false
      }
    })
    setSelectedCustomer(undefined)
  }

  useEffect(() => {
    handleOnConfirm(1)
    setSidebarTitle(selectedCustomer ? "Masukan Jumlah" : "Pilih Nama")
  }, [selectedCustomer])

  const { data: customers } = useGetAllCustomerByProps();
  const mutator = usePostUpdateCustomer()


  return (
    <>
      {selectedCustomer ? (
        <VirtualKeypad
          handleOnConfirm={handleOnConfirm}
        />
      )
        : (
          <CustomerPicker
            customers={customers ?? []}
            onSelectedCustomer={(customer) => setSelectedCustomer(customer)}
          />
        )}

    </>
  )
}
