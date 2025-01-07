using System.Globalization;
using System.Security.Claims;
using AMS.AppIdentity;
using AMS.Extension;
using AMS.Model;
using Microsoft.AspNetCore.Identity;
using Microsoft.Data.Sqlite;
using SqlKata;
using static AMS.Extension.UtilityExtension;

namespace AMS.StandardIdentity;

/// <summary>
/// Class that implement UserStoreBase, and IUserStore with TKey = int
/// </summary>
/// <typeparam name="TUser"></typeparam>
/// <typeparam name="TRole"></typeparam>
public class StandardUserStore<TUser, TRole>
    : UserStoreBase<
        TUser,
        TRole,
        int,
        IdentityUserClaim<int>,
        IdentityUserRole<int>,
        IdentityUserLogin<int>,
        IdentityUserToken<int>,
        IdentityRoleClaim<int>
    >,
        IUserStore<TUser>
    where TUser : IdentityUser<int>
    where TRole : IdentityRole<int>
{
    private async Task CreateConnection(Func<SqliteConnection, Task> connection)
    {
        using (var conn = _conn.CreateConnection())
        {
            await connection(conn);
        }
    }

    private readonly ISqliteConnectionProvider _conn;

    private async Task<IEnumerable<TUser>> ListUser()
    {
        IEnumerable<TUser>? users = null;
        var GetUsers_Query = new Query(nameof(User));
        await CreateConnection(async conn =>
        {
            users = (await conn.QuerySqlKataAsync<User>(GetUsers_Query)) as IEnumerable<TUser>;
        });

        return users;
    }

    public override IQueryable<TUser> Users =>
        Task.Run(async () => await ListUser()).Result.AsQueryable();

    public StandardUserStore(
        ISqliteConnectionProvider sqliteConnectionProvider,
        IdentityErrorDescriber describer
    )
        : base(describer)
    {
        _conn = sqliteConnectionProvider;
    }

    public override async Task<IdentityResult> CreateAsync(
        TUser user,
        CancellationToken cancellationToken = default
    )
    {
        var result = IdentityResult.Failed(
            new[] { new IdentityError { Description = "Email Already Exists!!" } }
        );
        // var CheckIfEmailExists_Query = new Query(nameof(IdentityUserIntKey)).Where(
        //     nameof(IdentityUserIntKey.NormalizedEmail),
        //     user.NormalizedEmail
        // );
        await CreateConnection(async conn =>
        {
            await conn.InsertToDatabase<User>(user as User, true);
            result = IdentityResult.Success;
        });

        return result;
    }

    public override async Task<IdentityResult> DeleteAsync(
        TUser user,
        CancellationToken cancellationToken = default
    )
    {
        var result = IdentityResult.Failed();

        var CheckIfUserExist_Query = new Query(nameof(User)).Where(
            FullNameof(nameof(User.Id)),
            user.Id
        );
        var DeleteUser_Query = new Query(nameof(User))
            .Where(FullNameof(nameof(User.Id)), user.Id)
            .AsDelete();

        await CreateConnection(async conn =>
        {
            var userExists = await conn.QuerySingleSqlKataAsync<User>(CheckIfUserExist_Query);
            if (userExists != null)
            {
                await conn.ExecuteSqlKataAsync(DeleteUser_Query);
                result = IdentityResult.Success;
            }
        });

        return result;
    }

    public override async Task<TUser?> FindByEmailAsync(
        string normalizedEmail,
        CancellationToken cancellationToken = default
    )
    {
        TUser? user = null;
        var FindByEmail_Query = new Query(nameof(User)).Where(
            nameof(User.NormalizedEmail),
            normalizedEmail
        );

        await CreateConnection(async conn =>
        {
            user = (await conn.QuerySingleSqlKataAsync<User>(FindByEmail_Query)) as TUser;
        });
        return user;
    }

    public override async Task<TUser?> FindByIdAsync(
        string userId,
        CancellationToken cancellationToken = default
    )
    {
        TUser? user = null;
        var FindById_Query = new Query(nameof(User)).Where(nameof(User.Id), userId);

        await CreateConnection(async conn =>
        {
            user = (await conn.QuerySingleSqlKataAsync<User>(FindById_Query)) as TUser;
        });
        return user;
    }

    public override async Task<TUser?> FindByNameAsync(
        string normalizedUserName,
        CancellationToken cancellationToken = default
    )
    {
        TUser? user = null;
        var FindById_Query = new Query(nameof(User)).Where(
            nameof(User.NormalizedUserName),
            normalizedUserName
        );

        await CreateConnection(async conn =>
        {
            user = (await conn.QuerySingleSqlKataAsync<User>(FindById_Query)) as TUser;
        });
        return user;
    }

    // Claim
    public override async Task<IList<Claim>> GetClaimsAsync(
        TUser user,
        CancellationToken cancellationToken = default
    )
    {
        var GetClaimsByUserId_Query = new Query(nameof(UserClaim)).Where(
            nameof(UserClaim.UserId),
            user.Id
        );

        var listClaims = new List<Claim>();
        await CreateConnection(async conn =>
        {
            var claims = await conn.QuerySqlKataAsync<UserClaim>(GetClaimsByUserId_Query);
            listClaims = claims.Select(claim => claim.ToClaim()).ToList();
        });

        return listClaims;
    }

    public override async Task AddClaimsAsync(
        TUser user,
        IEnumerable<Claim> claims,
        CancellationToken cancellationToken = default
    )
    {
        var CheckIfClaimExist_Query = new Query(nameof(UserClaim));

        await CreateConnection(async conn =>
        {
            foreach (var claim in claims)
            {
                var userClaim = CreateUserClaim(user, claim);
                var claimExist = await conn.QuerySingleSqlKataAsync<UserClaim>(
                    CheckIfClaimExist_Query
                        .Where(nameof(UserClaim.ClaimType), userClaim.ClaimType)
                        .Where(nameof(Model.UserClaim.UserId), user.Id)
                );

                if (claimExist == null)
                {
                    await conn.InsertToDatabase(userClaim, true);
                }
            }
        });
    }

    public override async Task ReplaceClaimAsync(
        TUser user,
        Claim claim,
        Claim newClaim,
        CancellationToken cancellationToken = default
    )
    {
        var constraint = new Dictionary<string, string>
        {
            { nameof(UserClaim.ClaimType), newClaim.Type },
            { nameof(UserClaim.UserId), user.Id.ToString() },
        };

        var IsClaimExist_Query = new Query(nameof(UserClaim)).Where(constraint);

        await CreateConnection(async conn =>
        {
            var claimInDb = await conn.QuerySingleSqlKataAsync<UserClaim>(IsClaimExist_Query);
            if (claimInDb != null)
            {
                await conn.UpdateByCondition(
                    base.CreateUserClaim(user, newClaim),
                    query =>
                    {
                        query.Where(constraint);
                    },
                    false,
                    typeof(UserClaim)
                );
            }
        });
    }

    public override async Task RemoveClaimsAsync(
        TUser user,
        IEnumerable<Claim> claims,
        CancellationToken cancellationToken = default
    )
    {
        var RemoveClaim_Query = new Query(nameof(UserClaim))
            .Where(nameof(UserClaim.UserId), user.Id)
            .WhereIn(nameof(UserClaim.ClaimType), claims.Select(claim => claim.Type));

        await CreateConnection(async conn =>
        {
            await conn.ExecuteSqlKataAsync(RemoveClaim_Query);
        });
    }

    // UserLogin
    public override async Task AddLoginAsync(
        TUser user,
        UserLoginInfo login,
        CancellationToken cancellationToken = default
    )
    {
        var CheckIfUserLoginExists_Query = new Query(nameof(UserLogin)).Where(
            nameof(UserLogin.UserId),
            user.Id
        );
        await CreateConnection(async conn =>
        {
            var userLoginExists = await conn.QuerySingleSqlKataAsync<UserLogin>(
                CheckIfUserLoginExists_Query
            );
            if (userLoginExists == null)
            {
                await conn.InsertToDatabase(CreateUserLogin(user, login), true);
            }
        });
    }

    public override async Task RemoveLoginAsync(
        TUser user,
        string loginProvider,
        string providerKey,
        CancellationToken cancellationToken = default
    )
    {
        var constraint = new Dictionary<string, string>
        {
            { nameof(UserLogin.UserId), user.Id.ToString() },
            { nameof(UserLogin.LoginProvider), loginProvider },
            { nameof(UserLogin.ProviderKey), providerKey },
        };

        var DeleteUserLogin_Query = new Query(nameof(UserLogin)).Where(constraint).AsDelete();

        await CreateConnection(async conn =>
        {
            await conn.ExecuteSqlKataAsync(DeleteUserLogin_Query);
        });
    }

    public override async Task<IList<UserLoginInfo>> GetLoginsAsync(
        TUser user,
        CancellationToken cancellationToken = default
    )
    {
        var GetLoginForUser_Query = new Query(nameof(UserLogin)).Where(
            nameof(UserLogin.UserId),
            user.Id
        );
        var listUserLoginInfo = new List<UserLoginInfo>();

        await CreateConnection(async conn =>
        {
            listUserLoginInfo = (await conn.QuerySqlKataAsync<UserLogin>(GetLoginForUser_Query))
                .Select(loginInfo => new UserLoginInfo(
                    loginInfo.LoginProvider,
                    loginInfo.ProviderKey,
                    loginInfo.ProviderDisplayName
                ))
                .ToList();
        });
        return listUserLoginInfo;
    }

    protected override async Task<IdentityUserLogin<int>?> FindUserLoginAsync(
        int userId,
        string loginProvider,
        string providerKey,
        CancellationToken cancellationToken
    )
    {
        var constraint = new Dictionary<string, string>
        {
            { nameof(UserLogin.LoginProvider), loginProvider },
            { nameof(UserLogin.ProviderKey), providerKey },
            { nameof(UserLogin.UserId), userId.ToString() },
        };
        var GetUserLogin_Query = new Query(nameof(UserLogin)).Where(constraint);

        IdentityUserLogin<int>? userLogin = null;

        await CreateConnection(async conn =>
        {
            userLogin =
                (await conn.QuerySingleSqlKataAsync<UserLogin>(GetUserLogin_Query))
                as IdentityUserLogin<int>;
        });

        return userLogin;
    }

    protected override async Task<IdentityUserLogin<int>?> FindUserLoginAsync(
        string loginProvider,
        string providerKey,
        CancellationToken cancellationToken
    )
    {
        var GetUserLogin_Query = new Query(nameof(UserLogin))
            .Where(nameof(UserLogin.LoginProvider), loginProvider)
            .Where(nameof(UserLogin.ProviderKey), providerKey);

        IdentityUserLogin<int>? userLogin = null;

        await CreateConnection(async conn =>
        {
            userLogin =
                (await conn.QuerySingleSqlKataAsync<UserLogin>(GetUserLogin_Query))
                as IdentityUserLogin<int>;
        });

        return userLogin;
    }

    // Role

    public override async Task AddToRoleAsync(
        TUser user,
        string normalizedRoleName,
        CancellationToken cancellationToken = default
    )
    {
        var GetUserRole_Query = new Query(nameof(Role)).Where(
            nameof(IdentityRole.NormalizedName),
            normalizedRoleName
        );

        await CreateConnection(async conn =>
        {
            var roleEntity = await conn.QuerySingleSqlKataAsync<TRole>(GetUserRole_Query);
            if (roleEntity == null)
            {
                throw new InvalidOperationException(
                    string.Format(
                        CultureInfo.CurrentCulture,
                        "NormalizedName Not Found",
                        normalizedRoleName
                    )
                );
            }

            await conn.InsertToDatabase(CreateUserRole(user, roleEntity), true, typeof(UserRole));
        });
    }

    public override async Task RemoveFromRoleAsync(
        TUser user,
        string normalizedRoleName,
        CancellationToken cancellationToken = default
    )
    {
        var GetRoleForNormalizedRoleName_Query = new Query(nameof(Role)).Where(
            nameof(Role.NormalizedName),
            normalizedRoleName
        );

        await CreateConnection(async conn =>
        {
            var role = await conn.QuerySingleSqlKataAsync<Role>(GetRoleForNormalizedRoleName_Query);
            if (role != null)
            {
                var IsRoleExistForUser_Query = new Query(nameof(UserRole))
                    .Where(nameof(UserRole.UserId), user.Id)
                    .Where(nameof(UserRole.RoleId), role.Id);

                var userRole = await conn.QuerySingleSqlKataAsync<UserRole>(
                    IsRoleExistForUser_Query
                );

                if (userRole != null)
                {
                    await conn.ExecuteSqlKataAsync(IsRoleExistForUser_Query.AsDelete());
                }
            }
        });
    }

    public override async Task<IList<string>> GetRolesAsync(
        TUser user,
        CancellationToken cancellationToken = default
    )
    {
        var roles = new List<string>();

        var GetRoles_Query = new Query(nameof(UserRole))
            .Join(nameof(Role), FullNameof(nameof(Role.Id)), FullNameof(nameof(UserRole.RoleId)))
            .Where(FullNameof(nameof(UserRole.UserId)), user.Id.ToString())
            .SelectAllClassProperties(typeof(Role));

        await CreateConnection(async conn =>
        {
            roles = (await conn.QuerySqlKataAsync<Role>(GetRoles_Query))
                .Select(role => role.NormalizedName)
                .ToList();
        });
        return roles;
    }

    public override async Task<bool> IsInRoleAsync(
        TUser user,
        string normalizedRoleName,
        CancellationToken cancellationToken = default
    )
    {
        bool userInRole = false;
        var GetRoles_Query = new Query(nameof(Role)).Where(
            nameof(Role.NormalizedName),
            normalizedRoleName
        );

        await CreateConnection(async conn =>
        {
            var role = await conn.QuerySingleSqlKataAsync<Role>(GetRoles_Query);
            if (role != null)
            {
                var DoesUserAlreadyOnRole_Query = new Query(nameof(UserRole))
                    .Where(nameof(UserRole.RoleId), role.Id)
                    .Where(nameof(UserRole.UserId), user.Id);

                var userRole = await conn.QuerySingleSqlKataAsync<UserRole>(
                    DoesUserAlreadyOnRole_Query,
                    false
                );
                userInRole = userRole != null ? true : false;
            }
        });

        return userInRole;
    }

    public override async Task<IList<TUser>> GetUsersForClaimAsync(
        Claim claim,
        CancellationToken cancellationToken = default
    )
    {
        var users = new List<User>();
        var constraint = new Dictionary<string, string>
        {
            { FullNameof(nameof(UserClaim.ClaimType)), claim.Type },
            { FullNameof(nameof(UserClaim.ClaimValue)), claim.Value },
        };
        var GetUserForClaim_Query = new Query(nameof(User))
            .Join(
                nameof(UserClaim),
                FullNameof(nameof(UserClaim.UserId)),
                FullNameof(nameof(User.Id))
            )
            .Where(constraint)
            .SelectAllClassProperties(typeof(User));

        await CreateConnection(async conn =>
        {
            users = (await conn.QuerySqlKataAsync<User>(GetUserForClaim_Query)).ToList();
        });

        return users as List<TUser>;
    }

    public override async Task<IList<TUser>> GetUsersInRoleAsync(
        string normalizedRoleName,
        CancellationToken cancellationToken = default
    )
    {
        var users = new List<User>();
        var GetUserInRole_Query = new Query(nameof(User))
            .Join(
                nameof(UserRole),
                FullNameof(nameof(UserRole.UserId)),
                FullNameof(nameof(User.Id))
            )
            .Join(nameof(Role), FullNameof(nameof(UserRole.RoleId)), FullNameof(nameof(Role.Id)))
            .Where(FullNameof(nameof(Role.NormalizedName), normalizedRoleName))
            .SelectAllClassProperties(typeof(User));

        await CreateConnection(async conn =>
        {
            users = (await conn.QuerySqlKataAsync<User>(GetUserInRole_Query)).ToList();
        });

        return users as List<TUser>;
    }

    public override async Task<IdentityResult> UpdateAsync(
        TUser user,
        CancellationToken cancellationToken = default
    )
    {
        await CreateConnection(async conn =>
        {
            await conn.UpdateByCondition(
                user,
                query =>
                {
                    query.Where(nameof(User.Id), user.Id);
                },
                false,
                typeof(User)
            );
        });
        return IdentityResult.Success;
    }

    protected override async Task AddUserTokenAsync(IdentityUserToken<int> token)
    {
        await CreateConnection(async conn =>
        {
            await conn.InsertToDatabase(token, true, typeof(UserToken));
        });
    }

    protected override async Task<TRole?> FindRoleAsync(
        string normalizedRoleName,
        CancellationToken cancellationToken
    )
    {
        TRole? role = null;
        var FindRole_Query = new Query(nameof(Role)).Where(
            nameof(Role.NormalizedName),
            normalizedRoleName
        );
        await CreateConnection(async conn =>
        {
            role = (await conn.QuerySingleSqlKataAsync<Role>(FindRole_Query)) as TRole;
        });
        return role;
    }

    protected override async Task<IdentityUserToken<int>?> FindTokenAsync(
        TUser user,
        string loginProvider,
        string name,
        CancellationToken cancellationToken
    )
    {
        UserToken? userToken = null;
        var constraint = new Dictionary<string, string>
        {
            { nameof(UserToken.UserId), user.Id.ToString() },
            { nameof(UserToken.LoginProvider), loginProvider },
            { nameof(UserToken.Name), name },
        };
        var FindToken_Query = new Query(nameof(UserToken)).Where(constraint);

        await CreateConnection(async conn =>
        {
            userToken = await conn.QuerySingleSqlKataAsync<UserToken>(FindToken_Query);
        });

        return userToken;
    }

    protected override async Task<TUser?> FindUserAsync(
        int userId,
        CancellationToken cancellationToken
    )
    {
        TUser? user = null;
        var FindUser_Query = new Query(nameof(User)).Where(nameof(User.Id), userId);

        await CreateConnection(async conn =>
        {
            user = (await conn.QuerySingleSqlKataAsync<User>(FindUser_Query)) as TUser;
        });
        return user;
    }

    protected override async Task<IdentityUserRole<int>?> FindUserRoleAsync(
        int userId,
        int roleId,
        CancellationToken cancellationToken
    )
    {
        IdentityUserRole<int>? role = null;
        var constraint = new Dictionary<string, string>
        {
            { nameof(UserRole.UserId), userId.ToString() },
            { nameof(UserRole.RoleId), roleId.ToString() },
        };
        var FindUserRole_Query = new Query(nameof(UserRole)).Where(constraint);

        await CreateConnection(async conn =>
        {
            role =
                (await conn.QuerySingleSqlKataAsync<UserRole>(FindUserRole_Query))
                as IdentityUserRole<int>;
        });
        return role;
    }

    protected override async Task RemoveUserTokenAsync(IdentityUserToken<int> token)
    {
        var RemoveUserToken_Query = new Query(nameof(UserToken)).Where(token);

        await CreateConnection(async conn =>
        {
            await conn.ExecuteSqlKataAsync(RemoveUserToken_Query);
        });
    }
}
