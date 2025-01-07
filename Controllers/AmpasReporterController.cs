using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace AMS.AmpasController;

[ApiController]
[Route("[controller]")]
[AllowAnonymous]
public class AmpasReporterController : Controller
{
    /*
     * TODO: Make Report endpoint
     * - this controller will create report like pdf or other document
     *   with printable format (nota, kuetansi dkk)
     * - make sure to save any of data produced by this controller
     *
     * */

    [HttpGet]
    [Route("ampas-report")]
    public async Task<ActionResult> GetAmpasReport([FromQuery] int amount)
    {
        return Ok();
    }
}
