import { Customer } from "@/bindings/Customer";
import { RetrieveData } from "@/bindings/RetrieveData";
import { generateCalendarObjectWithRetrieveData, ICalendarDateObject } from "@/utilities/generate-calendar-object";
import { useLayoutStore } from "@/utilities/layout-store";
import { Text, Avatar, Button, Center, Flex, Grid, Group, Stack, Title, Card } from "@mantine/core";
import { useViewportSize } from "@mantine/hooks";
import { addMonths, getDate, subMonths } from "date-fns";
import { ReactNode, useMemo } from "react";
import { FaRegCircleCheck } from "react-icons/fa6";

interface CalendarProps {
  date?: Date,
  retrievesData?: RetrieveData[],
  customer?: Customer,
  rightTopMenu?: () => ReactNode,
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
    onDateClick,
    rightTopMenu
  } = props;

  const { width, height } = useViewportSize();
  const allLayoutState = useLayoutStore(store => store.getAll);
  const isLayoutStateReady = useLayoutStore(store => store.isReady);

  const calendarObjs = useMemo(() => generateCalendarObjectWithRetrieveData(retrievesData, date), [date, retrievesData]);
  const calendarCellHeight = useMemo(() => {
    const { mainHeight } = allLayoutState();

    const dateLength = calendarObjs.dates.length;
    if (dateLength <= 28) return `${Math.round(mainHeight / 8)}px`;
    return calendarObjs.dates.length > 35 ? `${Math.round(mainHeight / 12.5)}px` : `${Math.round(mainHeight / 10)}px`
  }, [calendarObjs.dates, width, height, isLayoutStateReady])

  const pickCard = (dateObj: ICalendarDateObject) => (
    <Card
      withBorder
      shadow="sm"
      radius="md"
      mih={calendarCellHeight}
      onClick={() => onDateClick?.(dateObj.date)}
    >
      <Card.Section withBorder inheritPadding>
        <Group justify="space-between">
          <Group>
            {dateObj.retrieveData?.is_paid && (
              <FaRegCircleCheck color="teal" size={30} />
            )}
            <Text fw={500}>{getDate(dateObj.date)}</Text>
          </Group>
        </Group>
      </Card.Section>
      <Card.Section h={"100%"}>
        <Title ta={'center'} mt={15}>
          {dateObj.retrieveData?.amount}
        </Title>
      </Card.Section>
    </Card >
  )

  if (!isLayoutStateReady) return null;

  return (
    <Stack style={{ padding: '15px 20px' }}>
      <Flex
        align={'center'}
        justify={'space-between'}
        gap={20}
      >
        <Group>
          <Title miw={"300px"}>
            {calendarObjs.monthStr} {calendarObjs.year}
          </Title>

          {customer && (
            <>
              <Avatar size={'lg'}>
                {customer.customer_name.slice(0, 2).toUpperCase()}
              </Avatar>
              <Title>{customer.customer_name}</Title>
            </>
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
        </Group>

        <Group hidden={!!rightTopMenu}>
          {rightTopMenu?.()}
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
              {pickCard(dateObj)}
            </Grid.Col>
          ))
        }
      </Grid>
    </Stack>
  )
}
