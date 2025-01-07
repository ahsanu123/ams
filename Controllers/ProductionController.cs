using AMS.Model;
using AMS.Repository;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

namespace AMS.AmpasController;

[ApiController]
[Route("[controller]")]
public class ProductionController : Controller
{
    private IProductionRepository _productionRepo;
    private UserManager<User> _userManager;

    public ProductionController(IProductionRepository productionRepo, UserManager<User> userManager)
    {
        _productionRepo = productionRepo;
        _userManager = userManager;
    }

    [HttpPost]
    [Route("add-production")]
    public async Task<ActionResult> AddProduction([FromQuery] int amount, [FromBody] DateTime date)
    {
        var result = await _productionRepo.AddProduction(amount, date);
        return Ok(result);
    }

    [HttpPost]
    [Route("total-error")]
    public async Task<ActionResult> GetTotalError([FromBody] DateTime date)
    {
        var productionError = await _productionRepo.GetProductionTotalError(date);
        return Ok(productionError);
    }
}
