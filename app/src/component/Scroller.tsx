import { Box, Button, Heading, Text } from '@chakra-ui/react'
import { useRef } from 'react'
import { AiFillCaretDown, AiFillCaretUp } from 'react-icons/ai'
import './Scroller.css'

export interface ScrollerProps {
  title?: string,
  description?: string,
  children?: React.ReactNode
}
export default function Scroller(props: ScrollerProps) {

  const { children,
    title,
    description
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

  return (
    <Box>
      {title && <Heading>{title}</Heading>}
      {description && <Text>{description}</Text>}

      <sub>{description}</sub>
      <div className="scroller-container">
        <div
          className='content'
          ref={contentRef}
        >
          {children}
        </div>

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
      </div>
    </Box>
  )
}
