import "./GridButton.css"

interface GridButtonProps {
  buttons: string[]
  onClick: (key: string) => void
  breakpoint?: number
}

export default function GridButton(props: GridButtonProps) {
  const {
    buttons,
    onClick,
    breakpoint = 3,
  } = props;

  return (
    <div
      className="grid-button"
    >
      {
        buttons.map((item, index) => (
          <button
            key={`grid-btn-${index}`}
            onClick={() => onClick(item)}
          >
            {item}
          </button>
        ))
      }

    </div>
  )
}
