using AMS.Model;
using AMS.Repository;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

namespace AMS.AmpasController;

[ApiController]
[Route("[controller]")]
[AllowAnonymous]
public class AmpasPricingController : Controller
{
    private IAmpasRepository _ampasRepo;
    private UserManager<User> _userManager;

    public AmpasPricingController(IAmpasRepository ampasRepository, UserManager<User> userManager)
    {
        _ampasRepo = ampasRepository;
        _userManager = userManager;
    }

    [HttpGet]
    [Route("current-price")]
    public async Task<ActionResult> GetCurrentPrice()
    {
        var price = await _ampasRepo.GetCurrentPrice();
        return Ok(price);
    }

    [HttpGet]
    [Route("change-price")]
    public async Task<ActionResult> GetAmpasPricing([FromQuery] double price)
    {
        var success = await _ampasRepo.ChangeAmpasPrice(price);
        if (success)
            return Ok(new AMSResult { Success = true, Message = null });

        return NotFound();
    }

    [HttpPost]
    [Route("monthly-bill")]
    public async Task<ActionResult> GetMontlyBill([FromBody] DateTime date)
    {
        var user = await _userManager.FindByNameAsync(User.Identity.Name);
        if (user == null)
            return NotFound();
        var bill = await _ampasRepo.GetBillForUser(user, date);

        return Ok(bill);
    }

    [HttpPost]
    [Route("monthly-information")]
    [ProducesResponseType(typeof(List<AmpasModel>), 200)]
    public async Task<ActionResult> GetMontlyInformation([FromBody] DateTime date)
    {
        var user = await _userManager.FindByNameAsync(User.Identity.Name);
        if (user == null)
            return NotFound();

        var montlyInfo = (await _ampasRepo.GetListAmpasForUser(user, date)).ToList();

        return Ok(montlyInfo);
    }

    [HttpPost]
    [Route("set-paid-status")]
    public async Task<ActionResult> SetPaidStatus([FromBody] PaidStatus status)
    {
        var user = await _userManager.FindByNameAsync(status.Username);
        if (user == null)
            return NotFound();

        var result = await _ampasRepo.SetPaidStatus(user, status.From, status.To, status.Paid);

        return Ok(result);
    }
}
