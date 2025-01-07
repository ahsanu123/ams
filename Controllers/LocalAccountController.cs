using AMS.Model;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

[ApiController]
[Route("[controller]")]
public class LocalAccountController : Controller
{
    // TODO:
    // Register
    // Login
    // Logout

    private SignInManager<User> _signinManager;
    private UserManager<User> _userManager;

    public LocalAccountController(SignInManager<User> signInManager, UserManager<User> userManager)
    {
        _signinManager = signInManager;
        _userManager = userManager;
    }

    [HttpPost]
    [Route("register")]
    public async Task<ActionResult> Register([FromBody] User user, [FromQuery] string password)
    {
        var result = await _userManager.CreateAsync(user, password);
        return Ok(result);
    }

    [HttpPost]
    [Route("login-without-password")]
    public async Task<ActionResult> LoginWithoutPassword([FromQuery] string username)
    {
        var user = await _userManager.FindByNameAsync(username);
        if (user == null)
            return NotFound();
        await _signinManager.SignInAsync(user, false);
        return Ok();
    }

    [HttpPost]
    [Route("login")]
    public async Task<ActionResult> Login([FromQuery] string username, [FromBody] string password)
    {
        var user = await _userManager.FindByNameAsync(username);
        if (user == null)
            return NotFound();
        var result = await _signinManager.PasswordSignInAsync(user, password, false, false);
        return Ok(result);
    }

    [HttpGet]
    [Route("logout")]
    public async Task<ActionResult> Logout()
    {
        await _signinManager.SignOutAsync();
        return Ok();
    }

    [HttpGet]
    [Route("list-user")]
    public async Task<ActionResult> ListUser()
    {
        var users = _userManager.Users;
        return Ok(users.Select(user => user.UserName));
    }
}
