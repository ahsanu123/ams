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

 <img alt="licence" src="./ams-logo.svg">
 
# 🥔 Introduction

Ampas Management System (AMS) is a personal project built to manage the selling of soybean dregs (ampas). The application helps streamline and track the entire selling process.

AMS is a desktop application developed using Tauri, leveraging its lightweight, secure, and cross-platform capabilities. On the backend, AMS uses:

- 🚒 Diesel as a type-safe and efficient query builder.
- 🐟 Sea-query for handling flexible and easy-to-manage database migrations.

The goal of AMS is to provide a simple, fast, and reliable system tailored specifically for personal use in managing daily operations of soybean dregs sales.

## 🌳 Project Log

<details>
  <summary>Expand Me!!!</summary>

  
- 21 Januari 2025, running browser on raspberry pi zero 2 was to slow, even you don't start dotnet backend yet. after searching tauri seem solve this, but with consequence changing backend into tauri (rust).
- 24 Februari 2025, 👷‍♂️ TODO create diesel migration and sea-query table builder combination, create macro to automate sea-query table function creation from struct model
- 25 Februari 2025, turn out diesel was sync (because its made before async in rust), for current case its ok to use sync.
- 8 Maret 2025, Create Basic structure of CRUD with Rust in ProductRepository
- 8 Maret 2025, TODO: Implement DatabaseMetadata function with diesel.

</details>


## 🎏 Usefull Notes 

- running cargo test -> `cargo test test_insert_product -- --nocapture`
- get expanded macro for debugging -> `cargo expand --lib  model::product`
 
## ⚓ Reference 

- [Introducing the Identity API endpoints](https://andrewlock.net/exploring-the-dotnet-8-preview-introducing-the-identity-api-endpoints/)
- [ASP.NET Core Identity Github Folder](https://github.com/dotnet/aspnetcore/tree/main/src/Identity)
- [ASP.NET Core Identity Default Flow Implementation with Page](https://github.com/dotnet/aspnetcore/blob/main/src/Identity/UI/src/Areas/Identity/Pages/V5/Account/ExternalLogin.cshtml.cs)

<sub> Work In Progress, Made With ❤️ By Ah...</sub>
