import Calendar from "@/components/Calendar";
import { Box, Button, Text, Center, Divider, Flex, LoadingOverlay, Stack, Table, Title } from "@mantine/core";
import { FaCalendarAlt } from "react-icons/fa";
import { MdPayments } from "react-icons/md";
import { RadarChart } from '@mantine/charts';
import { usePaymentPageStore } from "./payment-page-store";
import { MonthPicker } from '@mantine/dates';
import { useEffect, useState } from "react";
import { SideBarComponentType, useSidebarStore } from "@/global-stores/right-sidebar-store";
import { useGetAllRetrieveData } from "@/api/v1/retrieve-data-controller/retrieve-data-controller";
import { formatDateTimeWithoutZ } from "@/utilities/date-to-string-without-z";
import { useGetAllBillingInfo } from "@/api/v1/billing-controller/billing-controller";
import { formatDateId } from "@/utilities/format-date-id";
import { formatAsRupiah } from "@/utilities/format-as-rupiah";
import { parseISO } from "date-fns";
import { PiEmptyDuotone } from "react-icons/pi";

export default function PaymentPage() {

  const [value, setValue] = useState<[string | null, string | null]>([null, null]);
  const isShowCalendar = usePaymentPageStore(store => store.isShowCalendar);
  const month = usePaymentPageStore(store => store.selectedMonth);

  const setIsShowCalendar = usePaymentPageStore(store => store.setIsShowCalendar);
  const increaseMonth = usePaymentPageStore(store => store.increaseMonth);
  const decreaseMonth = usePaymentPageStore(store => store.decreaseMonth);
  const customer = usePaymentPageStore(store => store.selectedCustomer);

  const setSidebarTitle = useSidebarStore(store => store.setTitle)
  const setSidebarComponent = useSidebarStore(store => store.setDisplayedComponent);

  const {
    data: retrievesData,
    refetch: refetchRetrievesData,
    isPending: retrievesDataIsPending
  } = useGetAllRetrieveData({
    customer_id: customer?.customer_id,
    month: formatDateTimeWithoutZ(month),
  }, {
    query: {
      enabled: !!customer
    }
  });

  const { data: billingInfos, isLoading: isGetBillingInfoLoading } = useGetAllBillingInfo({
    customer_id: customer?.customer_id,
    from: value[0] && formatDateTimeWithoutZ(parseISO(value[0])),
    to: value[1] && formatDateTimeWithoutZ(parseISO(value[1])),
  }, {
    query: {
      enabled: !!customer && !!value[1] && !!value[0]
    }
  });

  const billingInfoSummaryTable = () => {
    const billingInfo = billingInfos?.[0];
    if (!billingInfo) return null;

    return (
      <>
        <Title size='xl'>Ringkasan</Title>
        <Table
          variant="vertical"
          layout="fixed"
          withTableBorder
        >
          <Table.Tbody>
            <Table.Tr>
              <Table.Th w={160}>Dari tanggal</Table.Th>
              <Table.Td>{billingInfo.from && formatDateId(billingInfo.from)}</Table.Td>
            </Table.Tr>

            <Table.Tr>
              <Table.Th>Hingga tanggal</Table.Th>
              <Table.Td>{billingInfo.to && formatDateId(billingInfo.to)}</Table.Td>
            </Table.Tr>

            <Table.Tr>
              <Table.Th>Terbayar</Table.Th>
              <Table.Td>{formatAsRupiah(billingInfo.paid_bill)}</Table.Td>
            </Table.Tr>

            <Table.Tr>
              <Table.Th>Belum terbayar</Table.Th>
              <Table.Td>{formatAsRupiah(billingInfo.unpaid_bill)}</Table.Td>
            </Table.Tr>

            <Table.Tr>
              <Table.Th>Total ambil</Table.Th>
              <Table.Td>{billingInfo.amount}</Table.Td>
            </Table.Tr>
          </Table.Tbody>
        </Table>
      </>
    )
  }

  const retrievesDataRows = billingInfos?.map((info) => info.retrieve_data).flat().map((retrieveData) => (
    <Table.Tr key={retrieveData.retrieve_data_id}>
      <Table.Td>{formatDateId(retrieveData.date)}</Table.Td>
      <Table.Td>{retrieveData.amount}</Table.Td>
      <Table.Td>{formatAsRupiah(retrieveData.price.value)}</Table.Td>
      <Table.Td>{formatAsRupiah(retrieveData.amount * retrieveData.price.value)}</Table.Td>
    </Table.Tr>
  ));

  const data = [
    {
      product: 'Apples',
      'Sales January': 120,
      'Sales February': 100,
    },
    {
      product: 'Oranges',
      'Sales January': 98,
      'Sales February': 90,
    },
    {
      product: 'Tomatoes',
      'Sales January': 86,
      'Sales February': 70,
    },
    {
      product: 'Grapes',
      'Sales January': 99,
      'Sales February': 80,
    },
    {
      product: 'Bananas',
      'Sales January': 85,
      'Sales February': 120,
    },
    {
      product: 'Lemons',
      'Sales January': 65,
      'Sales February': 150,
    },
  ];
  const gridComponent = () => (
    <Flex>
      <Stack>
        <Title>Rentang Tanggal</Title>
        <MonthPicker
          size="xl"
          type="range"
          value={value}
          onChange={setValue}
        />
        <RadarChart
          h={300}
          data={data}
          dataKey="product"
          withPolarRadiusAxis
          withLegend
          series={[
            { name: 'Sales January', color: 'blue.6', opacity: 0.2 },
            { name: 'Sales February', color: 'orange.6', opacity: 0.2 },
          ]}
        />
      </Stack>

      <Divider size="md" orientation="vertical" />

      <Stack>
        <Box pos="relative">
          <LoadingOverlay
            visible={isGetBillingInfoLoading}
            zIndex={1000}
            overlayProps={{ radius: "sm", blur: 2 }}
          />
          {billingInfoSummaryTable()}
          {billingInfos && billingInfos.length > 0 ? (
            <Table.ScrollContainer minWidth={500} maxHeight={500}>
              <Table
                stickyHeader
                layout="fixed"
              >
                <Table.Thead>
                  <Table.Tr>
                    <Table.Th>Tanggal</Table.Th>
                    <Table.Th>Jumlah</Table.Th>
                    <Table.Th>Harga</Table.Th>
                    <Table.Th>Total</Table.Th>
                  </Table.Tr>
                </Table.Thead>
                <Table.Tbody>{retrievesDataRows}</Table.Tbody>
              </Table>
            </Table.ScrollContainer>
          ) : (
            <Center mih={"60vh"} miw={"50vw"}>
              <PiEmptyDuotone size={100} />
              <Text fw={'bolder'} >No Data</Text>
            </Center>
          )}
        </Box>
      </Stack>
    </Flex>
  )

  const rightTopMenu = () => (
    <Button.Group>
      <Button
        size="xl"
        variant={isShowCalendar ? "filled" : "default"}
        leftSection={<FaCalendarAlt />}
        onClick={() => setIsShowCalendar(true)}
      >
        Calendar
      </Button>

      <Button
        size="xl"
        variant={!isShowCalendar ? "filled" : "default"}
        leftSection={<MdPayments />}
        onClick={() => setIsShowCalendar(false)}
      >
        Payment
      </Button>
    </Button.Group>
  );


  useEffect(() => {
    setSidebarTitle("")
    setSidebarComponent(SideBarComponentType.PaymentPageCustomerPicker)
  }, [])

  useEffect(() => {
    refetchRetrievesData()
  }, [customer, month])

  return (
    <>
      <Box pos="relative">
        <LoadingOverlay
          visible={retrievesDataIsPending}
          zIndex={1000}
          overlayProps={{ radius: "sm", blur: 2 }}
        />

        <Calendar
          customer={customer}
          date={month}
          retrievesData={retrievesData ?? []}
          onNextMonth={() => increaseMonth()}
          onPrevMonth={() => decreaseMonth()}
          rightTopMenu={rightTopMenu}
          isShowDateGrid={isShowCalendar}
          gridComponent={gridComponent}
        />
      </Box>
    </>
  )
}
