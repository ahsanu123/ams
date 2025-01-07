using AMS.Constant;
using Microsoft.AspNetCore.Authorization;

namespace AMS.AppAuthorization;

public class SuperAdminAuthorizationRequirement : IAuthorizationRequirement
{
    public SuperAdminAuthorizationRequirement()
    {
        Email = Environment.GetEnvironmentVariable(AuthorizationConstant.SuperAdminEmail);
    }

    public string? Email { get; set; }
}
