using AMS.Constant;
using Microsoft.AspNetCore.Authentication;

namespace AMS.Services;

public static class AddExternalAuthenticationProviderBuilder
{
    public static AuthenticationBuilder AddExternalAuthenticationProvider(
        this AuthenticationBuilder builder,
        ConfigurationManager configuration
    )
    {
        builder
            .AddGoogle(option =>
            {
                option.ClientId = configuration[GoogleConstant.ClientId]!;
                option.ClientSecret = configuration[GoogleConstant.ClientSecret]!;
            })
            .AddGitHub(option =>
            {
                option.ClientId = configuration[GithubConstant.ClientId]!;
                option.ClientSecret = configuration[GithubConstant.ClientSecret]!;
            })
            .AddMicrosoftAccount(option =>
            {
                option.ClientId = configuration[MicrosoftConstant.ClientId]!;
                option.ClientSecret = configuration[MicrosoftConstant.ClientSecret]!;
            });

        return builder;
    }
}
