import { useEffect, useState } from "react"
import CampaignCard from "./CampaignCard"
import "./CampaignCardManager.css"
import { CampaignService, type CampaignModel } from "../api/generated"
import { defaultClient } from "../api/constant"


export default function CampaignCardManager() {

  const [campaigns, setCampaigns] = useState<CampaignModel[]>()

  const getCampaigns = async () => {
    const campaigns = await CampaignService.getCampaignCampaigns({ client: defaultClient })

    setCampaigns(campaigns.data as CampaignModel[])
  }

  const handleOnSave = async (data: CampaignModel) => {
    await CampaignService.postCampaignUpdateCampaign({
      client: defaultClient,
      body: data
    })
    window.location.reload()
  }

  const handleOnChange = (description: string, data: CampaignModel) => {
    const campaignData = campaigns?.find((campaign) => campaign.id === data.id)
    if (campaignData)
      campaignData.description = description

    if (campaigns)
      setCampaigns([...campaigns])
  }


  useEffect(() => {
    getCampaigns()
  }, []);

  return (
    <div
      className="campaign-card-manager"
    >
      {campaigns && campaigns.map((campaign) => (
        <CampaignCard
          data={campaign}
          onChange={handleOnChange}
          onSave={(data) => handleOnSave(data)}
        />
      ))}
    </div>
  )
}
