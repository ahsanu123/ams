using System.Security.Claims;
using AMS.Constant;
using AMS.Model;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;
using Newtonsoft.Json;

namespace AMS.AmpasController;

[ApiController]
[Route("[controller]")]
public class AccountController : Controller
{
    private SignInManager<User> _signinManager;
    private UserManager<User> _userManager;

    public AccountController(SignInManager<User> signinManager, UserManager<User> userManager)
    {
        _signinManager = signinManager;
        _userManager = userManager;
    }

    [HttpGet]
    [AllowAnonymous]
    [Route("ExternalLogin")]
    public async Task<IActionResult> ExternalLogin(string provider, string returnUrl = "/")
    {
        var externalLoginProvider = (
            await _signinManager.GetExternalAuthenticationSchemesAsync()
        ).FirstOrDefault(scheme => scheme.DisplayName?.ToLower() == provider.ToLower());

        if (externalLoginProvider == null)
        {
            return NotFound();
        }
        provider = externalLoginProvider.DisplayName!;

        var redirectUrl = Url.Action(
            "ExternalLoginCallback",
            "Account",
            new { ReturnUrl = returnUrl }
        );

        var properties = _signinManager.ConfigureExternalAuthenticationProperties(
            provider,
            redirectUrl
        );

        return new ChallengeResult(provider, properties);
    }

    [HttpGet]
    [AllowAnonymous]
    [Route("ExternalLoginCallback")]
    public async Task<IActionResult> ExternalLoginCallback(string? returnUrl = null)
    {
        var loginInfo = await _signinManager.GetExternalLoginInfoAsync();
        if (loginInfo == null)
            return NotFound();

        var email = loginInfo.Principal.FindFirstValue(ClaimTypes.Email);
        var username = loginInfo.Principal.FindFirstValue(ClaimTypes.Name);

        var user = await _userManager.FindByEmailAsync(email);
        var isUserNameExists = await _userManager.FindByNameAsync(username) != null;

        if (isUserNameExists && user == null)
            username += new Random().Next(1000, 9999);

        if (user == null)
        {
            var createResult = await _userManager.CreateAsync(
                new User
                {
                    UserName = username,
                    Email = email,
                    EmailConfirmed = true,
                }
            );
            if (!createResult.Succeeded)
                return BadRequest(createResult.Errors);
        }

        user = await _userManager.FindByEmailAsync(email);

        await _signinManager.SignInAsync(user, isPersistent: true);

        return Redirect(AuthorizationConstant.FrontendRedirectUrl);
    }

    [HttpGet]
    [Route("external-authentication-provider-info")]
    [Authorize(Policy = AuthorizationConstant.SuperAdminClaim)]
    public async Task<ActionResult> GetExternalAuthenticationProviderInfo()
    {
        var schemes = await _signinManager.GetExternalAuthenticationSchemesAsync();
        var providers = schemes.Select(scheme => scheme.DisplayName).ToList();

        return Ok(providers);
    }
}
