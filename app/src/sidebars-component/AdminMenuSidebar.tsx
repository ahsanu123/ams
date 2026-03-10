import { NavLink, Stack, Title } from "@mantine/core";
import { RiCustomerService2Fill } from "react-icons/ri";
import { IoMdPricetags } from "react-icons/io";
import { MdOutlinePayments } from "react-icons/md";
import { useNavigate } from "react-router";
import { AdminRoutes } from "@/routes";

export default function AdminMenuSidebar() {
  const navigate = useNavigate()

  return (
    <Stack>
      <NavLink
        variant="filled"
        active
        h={100}
        label={<Title>Payment</Title>}
        leftSection={<MdOutlinePayments size={50} />}
        onClick={() => navigate(`${AdminRoutes.AdminRoot}${AdminRoutes.PaymentPage}`)}
      />

      <NavLink
        variant="filled"
        active
        h={100}
        label={<Title>Customer Management</Title>}
        leftSection={<RiCustomerService2Fill size={50} />}
        onClick={() => navigate(`${AdminRoutes.AdminRoot}${AdminRoutes.CustomerManagementPage}`)}
      />
      <NavLink
        variant="filled"
        active
        h={100}
        label={<Title>Prices</Title>}
        leftSection={<IoMdPricetags size={50} />}
        onClick={() => navigate(`${AdminRoutes.AdminRoot}${AdminRoutes.PricesPage}`)}
      />
    </Stack>
  )
}
