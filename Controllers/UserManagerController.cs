using AMS.Constant;
using AMS.Model;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

namespace AMS.AmpasController;

[ApiController]
[Route("[controller]")]
[Authorize(Policy = AuthorizationConstant.SuperAdminClaim)]
public class UserManagerController : Controller
{
    private UserManager<User> _userManager;
    private SignInManager<User> _signinManager;

    public UserManagerController(UserManager<User> userManager, SignInManager<User> signInManager)
    {
        _userManager = userManager;
        _signinManager = signInManager;
    }

    [HttpPost]
    [Route("user-detail")]
    public ActionResult GetUserDetail([FromQuery] string id)
    {
        var user = _userManager.FindByIdAsync(id);
        return Ok(user);
    }

    [HttpPost]
    [Route("create-user")]
    public async Task<ActionResult> CreateUser([FromBody] User user)
    {
        var result = await _userManager.CreateAsync(user);
        return Ok(result);
    }

    [HttpGet]
    [Route("list-users")]
    public ActionResult GetAllUser()
    {
        var users = _userManager.Users;
        return Ok(users);
    }

    [HttpPost]
    [Route("update-user")]
    public async Task<ActionResult> UpdateUser([FromBody] User user)
    {
        var result = await _userManager.UpdateAsync(user);
        return Ok(result);
    }

    [HttpDelete]
    [Route("delete-user")]
    public async Task<ActionResult> DeleteUser([FromBody] User user)
    {
        var result = await _userManager.DeleteAsync(user);
        return Ok(result);
    }

    [HttpGet]
    [Route("sign-out")]
    [AllowAnonymous]
    public async Task<ActionResult> SignOutUser()
    {
        await _signinManager.SignOutAsync();
        return Ok();
    }
}
