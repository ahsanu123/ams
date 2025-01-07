using AMS.Constant;
using Microsoft.AspNetCore.Authorization;

namespace AMS.AppAuthorization;

public static class CustomAuthorizationExtension
{
    public static AuthorizationOptions AddCustomPolicies(this AuthorizationOptions option)
    {
        option.AddPolicy(
            PolicyConstant.SuperAdmin,
            policy =>
            {
                policy.AddRequirements(new SuperAdminAuthorizationRequirement());
            }
        );
        return option;
    }
}
