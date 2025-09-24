import './Scroller.css'

export interface ScrollerProps {
  onButtonUpClicked: () => void,
  onButtonDownClicked: () => void,
}
export default function Scroller(props: ScrollerProps) {

  const {
    onButtonDownClicked,
    onButtonUpClicked
  } = props

  return (
    <div className="scroller-container">
      <button
        onClick={onButtonUpClicked}
      >
        ⬆️
      </button>
      <button
        onClick={onButtonDownClicked}
      >
        ⬇️
      </button>
    </div>
  )
}
