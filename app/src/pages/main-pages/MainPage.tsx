import { useGetAllRetrieveData } from "@/api/v1/retrieve-data-controller/retrieve-data-controller";
import Calendar from "@/components/Calendar";
import { useEffect, useMemo, useState } from "react";
import { useMainPageStore } from "./main-page-store";
import { Box, LoadingOverlay } from "@mantine/core";
import { formatDateTimeWithoutZ } from "@/utilities/date-to-string-without-z";
import { useGetFirstCustomer } from "@/api/v1/customer-management-controller/customer-management-controller";

export default function MainPage() {
  const { data: firstCustomer } = useGetFirstCustomer();

  const selectedCustomer = useMainPageStore(store => store.selectedCustomer)
  const setSelectedCustomer = useMainPageStore(store => store.setSelectedCustomer)

  const month = useMemo(() => new Date(), [selectedCustomer]);

  const {
    data: retrievesData,
    refetch: refetchRetrievesData,
    isPending: retrievesDataIsPending
  } = useGetAllRetrieveData({
    customer_id: selectedCustomer?.customer_id,
    month: formatDateTimeWithoutZ(month),
  }, {
    query: {
      enabled: !!selectedCustomer
    }
  });

  useEffect(() => {
    refetchRetrievesData()
  }, [selectedCustomer])

  useEffect(() => {
    if (firstCustomer && !selectedCustomer)
      setSelectedCustomer(firstCustomer)
  }, [firstCustomer])

  return (
    <>
      <Box pos="relative">
        <LoadingOverlay
          visible={retrievesDataIsPending}
          zIndex={1000}
          overlayProps={{ radius: "sm", blur: 2 }}
        />

        <Calendar
          customer={selectedCustomer}
          date={month}
          retrievesData={retrievesData ?? []}
        />
      </Box>
    </>
  )
}
