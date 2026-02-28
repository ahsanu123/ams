import { Customer } from "@/bindings/Customer";
import { RetrieveData } from "@/bindings/RetrieveData";
import { generateCalendarObjectWithRetrieveData } from "@/utilities/generate-calendar-object";
import { Text, Avatar, Button, Center, Flex, Grid, Group, Stack, Title } from "@mantine/core";
import { addMonths, getDate, subMonths } from "date-fns";
import { useMemo } from "react";
import { FaRegCircleCheck } from "react-icons/fa6";

interface CalendarProps {
  date?: Date,
  retrievesData?: RetrieveData[],
  customer?: Customer,
  onPrevMonth?: (date: Date) => void,
  onNextMonth?: (date: Date) => void,
  onDateClick?: (date: Date) => void,
}

export default function Calendar(props: CalendarProps) {

  const {
    date = new Date(),
    retrievesData,
    customer,
    onPrevMonth,
    onNextMonth,
    onDateClick
  } = props;

  const calendarObjs = useMemo(() => generateCalendarObjectWithRetrieveData(retrievesData, date), [date, retrievesData]);
  const calendarCellHeight = useMemo(() => {
    const dateLength = calendarObjs.dates.length;
    if (dateLength <= 28) return "19vh";
    return calendarObjs.dates.length > 35 ? "12vh" : "15vh"
  }, [calendarObjs.dates])

  return (
    <Stack style={{ padding: '15px 20px' }}>
      <Flex
        align={'center'}
        justify={'start'}
        gap={20}
      >
        <Title miw={"300px"}>
          {calendarObjs.monthStr} {calendarObjs.year}
        </Title>

        {customer && (
          <Avatar size={'lg'}>
            {customer.customer_name.slice(0, 2).toUpperCase()}
          </Avatar>
        )}

        <Group
          hidden={!(onPrevMonth || onNextMonth)}
        >
          <Button
            size="xl"
            onClick={() => onPrevMonth?.(subMonths(date, 1))}
          >
            Prev
          </Button>
          <Button
            size="xl"
            onClick={() => onNextMonth?.(addMonths(date, 1))}
          >
            Next
          </Button>
        </Group>

      </Flex>

      <Grid columns={7}>
        {
          calendarObjs.staticDays.map((day, index) => (
            <Grid.Col
              key={index}
              span={1}
            >
              <Center>
                <Text
                  size="xl"
                  fw={'bold'}
                >
                  {day}
                </Text>
              </Center>
            </Grid.Col>
          ))
        }
        {
          calendarObjs.dates.map((dateObj, index) => (
            <Grid.Col
              key={index}
              span={1}
            >
              <Stack
                onClick={() => onDateClick?.(dateObj.date)}
                justify="space-between"
                style={{
                  padding: "5px",
                  minHeight: calendarCellHeight,
                  width: "100%",
                  border: "1px solid gray"
                }}
              >
                <Text fw={'bold'}>
                  {getDate(dateObj.date)}
                </Text>
                <Title ta={'center'}>
                  {dateObj.retrieveData?.amount}
                </Title>
                {dateObj.retrieveData?.is_paid && (
                  <FaRegCircleCheck color="teal" size={30} />
                )}
              </Stack>
            </Grid.Col>
          ))
        }
      </Grid>
    </Stack>
  )
}
