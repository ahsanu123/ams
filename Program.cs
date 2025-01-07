using AMS.AppAuthorization;
using AMS.InternalMigration;
using AMS.Model;
using AMS.Services;
using AMS.StandardIdentity;
using Microsoft.AspNetCore.Identity;

var builder = WebApplication.CreateBuilder(args);
var configuration = builder.Configuration;

var sqliteConnectionString = configuration.GetConnectionString("Sqlite");

builder.Services.AddControllers();

builder
    .Services.AddAuthentication()
    .AddCookie(IdentityConstants.BearerScheme)
    .AddExternalAuthenticationProvider(configuration);

builder
    .Services.AddIdentity<User, Role>(option =>
    {
        option.User.AllowedUserNameCharacters =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._@+ ";
        option.Password.RequireDigit = false;
        option.Password.RequireLowercase = false;
        option.Password.RequireUppercase = false;
        option.Password.RequireNonAlphanumeric = false;
        option.SignIn.RequireConfirmedAccount = false;
    })
    .AddRoles<Role>()
    .AddClaimsPrincipalFactory<CustomUserClaimsPrincipalFactory>()
    .AddStandardCustomIdentityStores()
    .AddDefaultTokenProviders();

builder.Services.AddAuthorization(option =>
{
    option.AddCustomPolicies();
});

builder.Services.AddFluentMigratorProvider(sqliteConnectionString!);

builder.Services.AddConfigurationProvider(configuration);
builder.Services.AddServicesCollection();

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(option =>
{
    option.UseAllOfForInheritance();
    // look at this
    // https://github.com/domaindrivendev/Swashbuckle.AspNetCore?tab=readme-ov-file#add-security-definitions-and-requirements
    // option.AddSecurityDefinition(
    //     "Bearer",
    //     new OpenApiSecurityScheme
    //     {
    //         In = ParameterLocation.Header,
    //         Description = "Please enter token",
    //         Name = "Authorization",
    //         Type = SecuritySchemeType.Http,
    //         BearerFormat = "JWT",
    //         Scheme = "bearer",
    //     }
    // );
});

builder.Services.AddCors(option =>
{
    option.AddDefaultPolicy(policy =>
    {
        policy.AllowCredentials();
        policy.AllowAnyMethod();
        policy.AllowAnyHeader();
        policy.SetIsOriginAllowed(origin => new Uri(origin).Host == "localhost");
    });
});

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseDeveloperExceptionPage();

    app.UseSwagger();
    app.UseSwaggerUI(option =>
    {
        option.EnableTryItOutByDefault();
    });
}

app.UseHttpsRedirection();
app.UseStaticFiles();
app.UseRouting();

app.UseCors();

app.UseCookiePolicy(new CookiePolicyOptions() { MinimumSameSitePolicy = SameSiteMode.None });

app.UseAuthentication();
app.UseAuthorization();

app.MapControllers();

app.UseFluentMigrator();
await app.AddDefaultRoles();
await app.AddDefaultAmpasPrice();
await app.AddDefaultUser();
app.Run();
