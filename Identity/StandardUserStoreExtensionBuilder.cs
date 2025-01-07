using AMS.Model;
using Microsoft.AspNetCore.Identity;

namespace AMS.StandardIdentity;

public static class StandardUserStoreExtensionBuilder
{
    /// <summary>
    /// a Static Function To Add Custom Store For IdentityUser with TKey = int
    /// </summary>
    /// <param name="IdentityBuilder"></param>
    /// <returns></returns>
    public static IdentityBuilder AddStandardCustomIdentityStores(this IdentityBuilder builder)
    {
        builder.Services.AddScoped<IUserStore<User>, StandardUserStore<User, Role>>();
        builder.Services.AddScoped<IRoleStore<Role>, StandardRoleStore<Role>>();
        return builder;
    }
}
