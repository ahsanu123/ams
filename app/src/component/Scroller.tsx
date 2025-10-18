import { useRef } from 'react'
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
    <>
      {title && <h1>{title}</h1>}
      {description && <sub>{description}</sub>}

      <sub>{description}</sub>
      <div className="scroller-container">
        <div
          className='content'
          ref={contentRef}
        >
          {children}
        </div>

        <div className='navigation-button'>
          <button
            onClick={() => onButtonUpClicked()}
          >
            ⬆️
          </button>
          <button
            onClick={() => onButtonDownClicked()}
          >
            ⬇️
          </button>
        </div>
      </div>
    </>
  )
}
