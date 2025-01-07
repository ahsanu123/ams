# 💩 Introduction

AMS - Ampas Management System is personal project for **personal use** for managing ampas selling.

## 🧻 Personal Note 

<details>
  <summary>Expand</summary>

```mermaid
classDiagram 

note for IAuthorizationService "AuthorizationService 
is Using AuthorizationHandler
to handle Requirement described 
by AuthorizationRequirement"

note for IAuthorizationHandler "register handler service 
with builder.Services.AddSingleton<IAuthorizationHandler, MinimumAgeHandler>();"

IAuthorizationService --> IAuthorizationHandler
IAuthorizationService --> IAuthorizationRequirement

IAuthorizationHandler --> IAuthorizationRequirement

class IAuthorizationService{
    AuthorizeAsync()
}

class IAuthorizationRequirement{
    Object ListRequirement
}

class IAuthorizationHandler{
    HandleRequirementAsync(IAuthorizationRequirement Requirement)
    HandleAsync(AuthorizationHandlerContext context)
}
```
</details>
 
## ⚓ Reference 

- [Introducing the Identity API endpoints](https://andrewlock.net/exploring-the-dotnet-8-preview-introducing-the-identity-api-endpoints/)
- [ASP.NET Core Identity Github Folder](https://github.com/dotnet/aspnetcore/tree/main/src/Identity)
- [ASP.NET Core Identity Default Flow Implementation with Page](https://github.com/dotnet/aspnetcore/blob/main/src/Identity/UI/src/Areas/Identity/Pages/V5/Account/ExternalLogin.cshtml.cs)

<sub> Work In Progress, Made With ❤️ By Ah...</sub>
