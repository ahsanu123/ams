<p>
  <a href="">
    <img alt="npm version" src="https://badgen.net/github/commits/ahsanu123/ams/">
  </a>
  <a href="">
    <img alt="npm" src="https://badgen.net/github/contributors/ahsanu123/ams/">
  </a>
  <a href="">
    <img alt="npm" src="https://badgen.net/github/branches/ahsanu123/ams/">
  </a>
  <a href="https://github.com/ahsanu123/ams/blob/main/LICENSE">
    <img alt="licence" src="https://badgen.net/github/license/ahsanu123/ams/">
  </a>
</p>

#  Introduction

AMS - Ampas Management System is personal project for **personal use** for managing soybean dregs selling.

## 🌳 Project Log

- 21 Januari 2025, running browser on raspberry pi zero 2 was to slow, even you don't start dotnet backend yet. after searching tauri seem solve this, but with consequence changing backend into tauri (rust).
- 24 Februari 2025, 👷‍♂️ TODO create diesel migration and sea-query table builder combination, create macro to automate sea-query table function creation from struct model
- 25 Februari 2025, turn out diesel was sync (because its made before async in rust), for current case its ok to use sync.

## 🎏 Usefull Notes 

- running cargo test -> `cargo test test_insert_product -- --nocapture`
- get expanded macro for debugging -> `cargo expand --lib  model::product`
 
## ⚓ Reference 

- [Introducing the Identity API endpoints](https://andrewlock.net/exploring-the-dotnet-8-preview-introducing-the-identity-api-endpoints/)
- [ASP.NET Core Identity Github Folder](https://github.com/dotnet/aspnetcore/tree/main/src/Identity)
- [ASP.NET Core Identity Default Flow Implementation with Page](https://github.com/dotnet/aspnetcore/blob/main/src/Identity/UI/src/Areas/Identity/Pages/V5/Account/ExternalLogin.cshtml.cs)

<sub> Work In Progress, Made With ❤️ By Ah...</sub>
