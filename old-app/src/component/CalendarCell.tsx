import { formatDateId, type ICalendarCell } from "@/utility";
import { Box, Center, Text } from "@chakra-ui/react";
import { isSameDay } from "date-fns";
import { FaRoadBarrier } from "react-icons/fa6";
import './CalendarCell.css';

interface CalendarCellProps {
  data: ICalendarCell,
  isAdmin?: boolean,
  onCellClicked?: (date?: Date) => void
}

export default function CalendarCellComponent(props: CalendarCellProps) {

  const {
    data,
    onCellClicked,
    isAdmin = false,
  } = props

  const highlightCurrentDay = data.date && isSameDay(data.date, new Date())

  const headerLabelComponent = () =>
    <Text fontWeight={'bold'}>{data.headerLabelText}</Text>

  const dateComponent = () => (
    <>
      {data.product
        ? (
          <Box
            onClick={() => isAdmin && onCellClicked?.(data.date)}
            className={`calendar-cell ${highlightCurrentDay ? 'highlight-current-day' : ''} ${isAdmin ? 'is-admin' : ''}`}
          >
            <Center className='current-day'>
              <Text>
                {data.date && formatDateId(data.date, "dd")}
              </Text>
            </Center>

            <Center className='taking-amount'>
              <Text
                textDecor={data.product.isPaid ? 'line-through' : ''}
                textStyle={'6xl'}
                fontWeight={'bold'}>
                {data.product?.amount}
              </Text>
            </Center>
          </Box>

        )
        : (
          <Box
            onClick={() => isAdmin && onCellClicked?.(data.date)}
            className={`calendar-cell ${highlightCurrentDay ? 'highlight-current-day' : ''} ${isAdmin ? 'is-admin' : ''}`}
          >
            <Center className='current-day'>
              <Text>
                {data.date && formatDateId(data.date, "dd")}
              </Text>
            </Center>

          </Box>
        )
      }
    </>
  )

  const hiddenDateComponent = () => (
    <Center
      alignItems={'center'}
      className="calendar-cell stripe"
    >
      <FaRoadBarrier size={'40px'} />
    </Center>
  )

  switch (data.type) {
    case 'HeaderLabel':
      return headerLabelComponent()

    case 'ShowDate':
      return dateComponent()

    case 'HiddenDate':
      return hiddenDateComponent()

    default:
      return <></>
  }
}
