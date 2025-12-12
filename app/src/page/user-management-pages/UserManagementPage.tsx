import { Box, Tabs } from '@chakra-ui/react';
import CreateNewCustomerTab from './components/CreateNewCustomerTab';
import UpdateCustomerTab from './components/UpdateCustomerTab';
import CustomerMoneyTab from './components/CustomerMoneyTab';
import 'react-simple-keyboard/build/css/index.css';
import './UserManagementPage.css';
import UpdateDregPriceTab from './components/UpdateDregPriceTab';

enum UserManagementTabs {
  CreateNewCustomer = "Create New Customer",
  UpdateCustomer = "Update Customer",
  CustomerMoneyManagement = "Customer Money",
  UpdateDregPrice = "Update Dreg Price"
}

export default function UserManagementPage() {

  return (
    <Box>
      <Tabs.Root defaultValue={UserManagementTabs.CreateNewCustomer}>
        <Tabs.List>

          <Tabs.Trigger value={UserManagementTabs.CreateNewCustomer}>
            {UserManagementTabs.CreateNewCustomer}
          </Tabs.Trigger>

          <Tabs.Trigger value={UserManagementTabs.UpdateCustomer}>
            {UserManagementTabs.UpdateCustomer}
          </Tabs.Trigger>

          <Tabs.Trigger value={UserManagementTabs.CustomerMoneyManagement}>
            {UserManagementTabs.CustomerMoneyManagement}
          </Tabs.Trigger>

          <Tabs.Trigger value={UserManagementTabs.UpdateDregPrice}>
            {UserManagementTabs.UpdateDregPrice}
          </Tabs.Trigger>

        </Tabs.List>

        <Tabs.Content
          value={UserManagementTabs.CreateNewCustomer}>
          <CreateNewCustomerTab />
        </Tabs.Content>

        <Tabs.Content
          value={UserManagementTabs.UpdateCustomer}>
          <UpdateCustomerTab />
        </Tabs.Content>

        <Tabs.Content
          value={UserManagementTabs.CustomerMoneyManagement}>
          <CustomerMoneyTab />
        </Tabs.Content>

        <Tabs.Content
          value={UserManagementTabs.UpdateDregPrice}>
          <UpdateDregPriceTab />
        </Tabs.Content>

      </Tabs.Root>
    </Box>
  )
}
