import { Customer } from "@/bindings/Customer";
import { Box, Button, ScrollArea, Title } from "@mantine/core";
import { useViewportSize } from "@mantine/hooks";

interface CustomerPickerProps {
  customers: Customer[],
  onSelectedCustomer?: (customer: Customer) => void,
}
export default function CustomerPicker(props: CustomerPickerProps) {
  const {
    customers,
    onSelectedCustomer
  } = props;

  const { height } = useViewportSize();

  return (
    <ScrollArea h={height * 0.52} overscrollBehavior="contain">
      {customers.map((customer, index) => (
        <Box
          key={index}
          mb={20}
        >
          <Button
            disabled={!onSelectedCustomer}
            onClick={() => onSelectedCustomer?.(customer)}
            fullWidth
            h={100}
          >
            <Title>
              {customer.customer_name}
            </Title>
          </Button>
        </Box>
      ))}
    </ScrollArea>
  )
}
