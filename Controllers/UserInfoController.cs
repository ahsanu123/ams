using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

[ApiController]
[Route("[controller]")]
public class UserInfoController : Controller
{
    [HttpGet]
    [Route("who-am-i")]
    [AllowAnonymous]
    public async Task<ActionResult> GetUserInformation()
    {
        return Ok(
            User.Claims.Select(claim => new { Type = claim.Type, Value = claim.Value }).ToList()
        );
    }
}
