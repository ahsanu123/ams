import { useGetAllCustomerByProps, usePostUpdateCustomer } from "@/api/v1/customer-management-controller/customer-management-controller";
import { Customer } from "@/api/v1/models";
import { usePostCreateRetrieveData } from "@/api/v1/retrieve-data-controller/retrieve-data-controller";
import CustomerPicker from "@/components/CustomerPicker";
import VirtualKeypad from "@/components/VirtualKeypad";
import { useSidebarStore } from "@/global-stores/right-sidebar-store";
import { notifications } from "@mantine/notifications";
import { useEffect, useState } from "react";
import { useMainPageStore } from "../main-page-store";

export default function MainPageCustomerPicker() {

  const [selectedCustomer, setSelectedCustomer] = useState<Customer | undefined>(undefined)
  const setSidebarTitle = useSidebarStore(store => store.setTitle)

  const setMainPageSelectedCustomer = useMainPageStore(store => store.setSelectedCustomer)

  const createRetrieveDataMutator = usePostCreateRetrieveData();

  const handleOnConfirm = async (amount: number) => {
    if (selectedCustomer?.customer_id === undefined) return;

    let result = await createRetrieveDataMutator.mutateAsync({
      data: {
        amount,
        customer_id: selectedCustomer.customer_id
      }
    })

    if (result > 0) notifications.show({
      message: <b>Berhasil</b>,
      color: 'green'
    })

    setMainPageSelectedCustomer(selectedCustomer);
    setSelectedCustomer(undefined)

  }

  const validatorFunction = (value: number): string | undefined => {
    if (value <= 0) return "Tidak Boleh Nol (0)!!"
    return undefined
  }

  useEffect(() => {
    setSidebarTitle(selectedCustomer ? "Masukan Jumlah" : "Pilih Nama")
  }, [selectedCustomer])

  const { data: customers } = useGetAllCustomerByProps();

  return (
    <>
      {selectedCustomer ? (
        <VirtualKeypad
          handleOnConfirm={(value) => handleOnConfirm(value)}
          validatorFunction={validatorFunction}
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
