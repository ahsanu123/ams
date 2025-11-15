import { Box, Button, Heading, Stack, Text } from '@chakra-ui/react'
import { useRef } from 'react'
import { AiFillCaretDown, AiFillCaretUp } from 'react-icons/ai'
import './Scroller.css'

export interface ScrollerProps {
  title?: string,
  leftNavigation?: boolean,
  description?: string,
  children?: React.ReactNode
}
export default function Scroller(props: ScrollerProps) {

  const { children,
    title,
    description,
    leftNavigation = false,
  } = props

  const contentRef = useRef<HTMLDivElement>(null)

  const onButtonUpClicked = () => {
    contentRef.current?.scrollBy({
      top: -50
    })
  }

  const onButtonDownClicked = () => {
    contentRef.current?.scrollBy({
      top: 50
    })
  }

  const navigationButton = () =>
    <div className='navigation-button'>
      <Button
        colorPalette={'teal'}
        onClick={() => onButtonUpClicked()}
      >
        <AiFillCaretUp />
      </Button>
      <Button
        colorPalette={'teal'}
        onClick={() => onButtonDownClicked()}
      >
        <AiFillCaretDown />
      </Button>
    </div>

  return (
    <Box>
      {title && <Heading>{title}</Heading>}
      {description && <Text>{description}</Text>}

      <sub>{description}</sub>
      <div className="scroller-container">
        {leftNavigation && navigationButton()}

        <Stack
          gap={5}
          className='content'
          ref={contentRef}
        >
          {children}
        </Stack>

        {!leftNavigation && navigationButton()}

      </div>
    </Box>
  )
}
