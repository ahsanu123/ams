using System.Security.Claims;
using AMS.Model;
using AMS.Repository;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

namespace AMS.AmpasController;

[ApiController]
[Route("[controller]")]
// [Authorize(
// Roles = $"{RoleConstant.GeneralAdmin},{RoleConstant.AsiaAdmin},{RoleConstant.AmericanAdmin},{RoleConstant.EuropeAdmin}"
// )]
public class CampaignController : Controller
{
    private ICampaignRepository _campaignRepo;
    private UserManager<User> _userManager;

    public CampaignController(ICampaignRepository campaignRepository, UserManager<User> userManager)
    {
        _campaignRepo = campaignRepository;
        _userManager = userManager;
    }

    /// <summary>
    /// Create New Campaign, Make Sure To Use Role to <paramref name="campaignModel"/> Type
    /// </summary>
    /// <param name="CampaignModel"></param>
    /// <returns></returns>
    [HttpPost]
    [Route("create-campaign")]
    public async Task<ActionResult> CreateCampaign([FromBody] CampaignModel campaignModel)
    {
        campaignModel.Type = campaignModel.Type.ToUpper();
        await _campaignRepo.CreateCampaign(campaignModel);
        return Ok();
    }

    [HttpPost]
    [Route("update-campaign")]
    public async Task<ActionResult> UpdateCampaign([FromBody] CampaignModel campaignModel)
    {
        await _campaignRepo.UpdateCampaign(campaignModel);
        return Ok();
    }

    [HttpGet]
    [Route("campaigns")]
    public async Task<ActionResult> GetCampaigns()
    {
        var campaigns = new List<CampaignModel>();

        var roles = User.Claims.Where(claim => claim.Type == ClaimTypes.Role).ToList();
        campaigns = (await _campaignRepo.GetCampaigns(roles)).ToList();

        return Ok(campaigns);
    }
}
