import "./CampaignCard.css"
import type { CampaignModel } from "../api/generated"

interface CampaignCardProps {
  data: CampaignModel
  onChange: (description: string, data: CampaignModel) => void
  onSave: (data: CampaignModel) => void
}

export default function CampaignCard(props: CampaignCardProps) {
  const {
    data,
    onChange,
    onSave
  } = props;
  return (
    <div
      className="campaign-card"
    >
      <h4> 🌏 {data.title}</h4>
      <sub>
        Short Description 🌎 🌏 🌍
      </sub>

      <hr />

      <textarea
        onChange={(target) => onChange(target.target.value, data)}
        id="textarea"
        rows={8}
        value={data.description ?? ""}
      />

      <button
        onClick={() => onSave(data)}
      >
        Save
      </button>

    </div>
  )
}
