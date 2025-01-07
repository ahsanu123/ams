using AMS.Constant;
using AMS.Model;
using AMS.Repository;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

namespace AMS.AmpasController;

[ApiController]
[Route("[controller]")]
[Authorize]
public class AmpasDailyController : Controller
{
    /*
     * TODO: Make Daily Ampas Utility here
     * - AddAmpasAmount -> every day ampas amount
     * - DailySummary -> show how many produced ampas and how many taken ampas by buyer
     *
     * */

    private IAmpasRepository _ampasRepo;
    private UserManager<User> _userManager;

    public AmpasDailyController(IAmpasRepository ampasRepository, UserManager<User> userManager)
    {
        _ampasRepo = ampasRepository;
        _userManager = userManager;
    }

    [HttpGet]
    [Route("add-ampas")]
    public async Task<IActionResult> AddAmpasAmount([FromQuery] int amount)
    {
        var user = await _userManager.FindByNameAsync(User.Identity.Name ?? "");
        if (user == null)
            return NotFound();

        var result = await _ampasRepo.AddAmpasForUser(user, amount);

        return Ok();
    }

    [HttpPost]
    [Route("summary-info")]
    [ProducesResponseType(typeof(AmpasSummary), 200)]
    public async Task<IActionResult> GetDailySummary(
        [FromQuery] AmpasSummaryDuration duration,
        [FromBody] DateTime? date = null
    )
    {
        var datetime = date ?? DateTime.Now;
        var summary = await _ampasRepo.GetSummary(datetime, duration);

        return Ok(summary);
    }

    [HttpPost]
    [Route("user-summary-info")]
    [ProducesResponseType(typeof(AmpasSummary), 200)]
    public async Task<IActionResult> GetUserSummary(
        [FromQuery] AmpasSummaryDuration duration,
        [FromQuery] string username,
        [FromBody] DateTime? date = null
    )
    {
        var user = await _userManager.FindByNameAsync(username);
        if (user == null)
            return NotFound();

        var datetime = date ?? DateTime.Now;
        var summary = await _ampasRepo.GetSummaryForUser(datetime, user, duration);

        return Ok(summary);
    }
}
