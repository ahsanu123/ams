import { useGetAllCustomerByProps } from "@/api/v1/customer-management-controller/customer-management-controller";
import CustomerPicker from "@/components/CustomerPicker";
import { usePaymentPageStore } from "@/pages/payment-page/payment-page-store";

export default function PaymentPageCustomerPicker() {
  const { data: customers } = useGetAllCustomerByProps();
  const setCustomer = usePaymentPageStore(store => store.setSelectedCustomer);

  return (
    <>
      <CustomerPicker
        customers={customers ?? []}
        onSelectedCustomer={(customer) => setCustomer(customer)}
      />
    </>
  )
}
