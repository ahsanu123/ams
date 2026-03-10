import "./GridButton.css"

interface GridButtonProps {
  buttons: string[]
  onClick: (key: string) => void
}

export default function GridButton(props: GridButtonProps) {
  const {
    buttons,
    onClick,
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
