using AMS.Constant;
using AMS.Model;
using Microsoft.AspNetCore.Identity;

namespace AMS.Services;

public static class AddDefaultUserBuilder
{
    public static async Task<IApplicationBuilder> AddDefaultUser(this IApplicationBuilder builder)
    {
        using var scope = builder.ApplicationServices.CreateScope();

        var roleManager = scope.ServiceProvider.GetService<RoleManager<Role>>();
        var userManager = scope.ServiceProvider.GetService<UserManager<User>>();

        var users = new List<string>()
        {
            "pak lurah",
            "mas sinin",
            "mas tresno",
            "mas misbah",
            "admin",
        };

        if (roleManager == null || userManager == null)
            throw new ArgumentNullException("Role Manager or User Manager was Null!!");

        foreach (var username in users)
        {
            var userExists = await userManager.FindByNameAsync(username);
            if (userExists == null)
            {
                var newUser = new User { UserName = username };
                // set default passowrd to username underscore and 123
                await userManager.CreateAsync(newUser, $"{username.Replace(" ", "_")}123");
            }

            var userInDb = await userManager.FindByNameAsync(username);
            if (userInDb == null)
                throw new Exception("User in db is Null");

            var alreadyInrole = await userManager.IsInRoleAsync(userInDb, RoleConstant.Buyer);
            if (!alreadyInrole && username != "admin")
                await userManager.AddToRoleAsync(userInDb, RoleConstant.Buyer);

            if (!alreadyInrole && username == "admin")
                await userManager.AddToRoleAsync(userInDb, RoleConstant.LocalSuperAdmin);
        }

        return builder;
    }
}
