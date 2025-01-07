using AMS.Model;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

namespace AMS.AmpasController;

[ApiController]
[Route("[controller]")]
// [Authorize(Policy = AuthorizationConstant.SuperAdminClaim)]
public class RoleManagerController : Controller
{
    private RoleManager<Role> _roleManager;
    private UserManager<User> _userManager;

    public RoleManagerController(RoleManager<Role> roleManager, UserManager<User> userManager)
    {
        _roleManager = roleManager;
        _userManager = userManager;
    }

    [HttpPost]
    [Route("add-role-for-email/{email}")]
    public async Task<ActionResult> AddRoleForUserByEmail(
        [FromRoute] string email,
        [FromBody] string roleName
    )
    {
        var role = await _roleManager.FindByNameAsync(roleName);
        var user = await _userManager.FindByEmailAsync(email);
        if (role != null && user != null)
        {
            var result = await _userManager.AddToRoleAsync(user, role.Name);
            return Ok(result);
        }

        return NotFound();
    }

    [HttpPost]
    [Route("remove-role-for-user/{userName}")]
    public async Task<ActionResult> RemoveRoleForUser(
        [FromRoute] string userName,
        [FromBody] string roleName
    )
    {
        var role = await _roleManager.FindByNameAsync(roleName);
        var user = await _userManager.FindByNameAsync(userName);
        if (role != null && user != null)
        {
            var result = await _userManager.RemoveFromRoleAsync(user, role.Name);
            return Ok(result);
        }

        return NotFound();
    }

    [HttpPost]
    [Route("add-role-for-user/{userName}")]
    public async Task<ActionResult> AddRoleForUser(
        [FromRoute] string userName,
        [FromBody] string roleName
    )
    {
        var role = await _roleManager.FindByNameAsync(roleName);
        var user = await _userManager.FindByNameAsync(userName);
        if (role != null && user != null)
        {
            var result = await _userManager.AddToRoleAsync(user, role.Name);
            return Ok(result);
        }

        return NotFound();
    }

    [HttpPost]
    [Route("create-role")]
    public async Task<ActionResult> CreateRole([FromBody] Role role)
    {
        var result = await _roleManager.CreateAsync(role);
        return Ok(result);
    }

    [HttpGet]
    [Route("get-roles")]
    public ActionResult GetRoles()
    {
        var roles = _roleManager.Roles;
        return Ok(roles);
    }

    [HttpGet]
    [Route("get-role-for-email/{email}")]
    public async Task<ActionResult> GetRoleForEmail([FromRoute] string email)
    {
        var user = await _userManager.FindByEmailAsync(email);
        if (user != null)
        {
            var roles = await _userManager.GetRolesAsync(user);
            return Ok(roles);
        }

        return NotFound();
    }

    [HttpGet]
    [Route("get-role-for-user/{userName}")]
    public async Task<ActionResult> GetRoleForUser([FromRoute] string userName)
    {
        var user = await _userManager.FindByNameAsync(userName);
        if (user != null)
        {
            var roles = await _userManager.GetRolesAsync(user);
            return Ok(roles);
        }

        return NotFound();
    }

    [HttpGet]
    [Route("is-user-in-role")]
    public async Task<ActionResult> IsUserInRole(
        [FromQuery] string userName,
        [FromQuery] string roleName
    )
    {
        var role = await _roleManager.FindByNameAsync(roleName);
        if (role == null)
            return NotFound();

        var user = await _userManager.FindByNameAsync(userName);
        if (user != null)
        {
            var result = await _userManager.IsInRoleAsync(user, role.Name!);
            return Ok(result);
        }
        return NotFound();
    }

    [HttpGet]
    [Route("get-user-for-role")]
    public async Task<ActionResult> GetUserForRole([FromQuery] string roleName)
    {
        var role = await _roleManager.FindByNameAsync(roleName);
        if (role == null)
            return NotFound();

        var users = await _userManager.GetUsersInRoleAsync(role.Name);
        return Ok(users);
    }
}
