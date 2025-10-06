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

 <img alt="licence" src="./docs/ams-logo.svg">

# 🥔 Introduction

Ampas Management System (AMS) is a personal project built to streamline and manage the selling process of soybean dregs (ampas). It simplifies daily operations by providing an efficient way to track sales and manage records, making day-to-day management easier and more organized.

AMS is designed as a desktop application powered by Tauri, offering a lightweight, secure, and cross-platform native experience. To accelerate development, the project is structured into three separate modules:
- Shared – Common data models, utilities, and types used across the system.
- API – A standalone backend service that can be developed and tested independently.
- Tauri – The desktop shell that packages the frontend and backend into a native app.

This modular setup allows frontend development to be done directly in the browser—interacting with the API via fetch—without needing to recompile Tauri for every change, significantly improving iteration speed.

### 🩼 Tech Stack

- 🐟 SeaORM – A modern, async ORM for Rust that simplifies database operations while maintaining type safety.
- 🌊 SeaQuery – Flexible and maintainable database migration management.
- ⚛️ React Router v7 – Provides modern routing, data APIs, and the Suspense feature for a clean, maintainable frontend architecture. 

 ### 🎯 Goal

The goal of AMS is to deliver a simple, fast, and reliable system tailored for personal use, focused on managing and tracking the daily sales operations of soybean dregs efficiently in limited resource device.

## 🌳 Project Log

<details>
  <summary>Expand Me!!!</summary>

```mermaid
flowchart TD
    UStart(start for user)-->UA
    UA[user select their name] --> UB[user insert how much they take]
    UB --> UC[UI get back to dashboard and showing data how much user take]

    AStart(start for admin) --> OpenHiddenMenu 
    OpenHiddenMenu[open hidden menu button and enter admin password]
    OpenHiddenMenu --> Menu{what menu does admin click? lets use tab UI}

    Menu --> UserPayment 
    Menu --> Report 
    Menu --> ChangeDregPrice
    UserPayment
    Report
    ChangeDregPrice

    UserPayment --> PaymentDescription[
        admin able to check user bill or 
        change payment status for some duration 
        for user
    ]

    Report --> ReportDescription[
        admin able to generate report for some duration 
        including total revenue, total production, 
        total bill for every user, and detail 
        table how many user take for each day including
        date detail
    ]

    ChangeDregPrice --> ChangeDregPriceDescription[
        admin able to change dreg price, dreg price 
        will record on database, and admin able to view all 
        record of dreg price
    ]

```

### 🎻 Current UI to Remind

![image](https://github.com/user-attachments/assets/c6b4eeda-bdfc-4d92-b971-1838f062692e)
![image](https://github.com/user-attachments/assets/2621bcb2-4840-4c7c-899c-2fdb72629f01)
![image](https://github.com/user-attachments/assets/09b325ff-29b9-4877-9374-c06066753e5a)
![image](https://github.com/user-attachments/assets/2cf524ac-f182-4dab-ac06-e91ccdbfd5d0)
![first test on real raspberrypi zero 2w device](./docs/build_alpha_1.jpg)
![first test on real raspberrypi zero 2w device](./docs/build_alpha_2.jpg)
  
- 21 Januari 2025, running browser on raspberry pi zero 2 was to slow, even you don't start dotnet backend yet. after searching tauri seem solve this, but with consequence changing backend into tauri (rust).
- 24 Februari 2025, 👷‍♂️ TODO create diesel migration and sea-query table builder combination, create macro to automate sea-query table function creation from struct model
- 25 Februari 2025, turn out diesel was sync (because its made before async in rust), for current case its ok to use sync.
- 8 Maret 2025, Create Basic structure of CRUD with Rust in ProductRepository
- 27 maret 2025, first trying on real raspberry pi zero 2w device, app is fast enough (compared to aspnet core and spa combination 😄), but authentication cookie still not work now, i think its because tauri permision, need reading more about it in their docs 
- ✔️ TODO: Implement DatabaseMetadata function with diesel.
- 🌳 TODO: create list of needed command for ui
- 🌳 TODO: create list of response name model for ui
- 🌳 TODO: create UI mockup
- 🌳 TODO: create needed query for command
- 🌳 TODO: create list logic for UI
- ✔️ TODO: add mobx -> Nope use Zustand instead
- ✔️ TODO: create password hashing with argon and save to db
- 🌳 TODO: create Dockerfile and documentation about cross compile for archlinuxarm in [CrossCompile](./docs/cross_compile.md)

</details>


## 🎏 Useful Notes 

- running cargo test -> `cargo test test_insert_product -- --nocapture`
- get expanded macro for debugging -> `cargo expand --lib  model::product`
- react router Revalidate for refetching data and route
- usestate with zustand to update ui only
- to run vitest with ui and watch mode `yarn test --ui --watch`
- make binary runable -> `sudo chmod +x binaryName`
- to debug output of macro run `dbg!(variable);`
- **quote!** repetition -> [repetition](https://docs.rs/quote/latest/quote/macro.quote.html#interpolation)
- swagger ui with utoipa actix swagger-ui `http://localhost:9090/swagger-ui/index.html`
 
## ⚓ Reference 

- [Introducing the Identity API endpoints](https://andrewlock.net/exploring-the-dotnet-8-preview-introducing-the-identity-api-endpoints/)
- [ASP.NET Core Identity Github Folder](https://github.com/dotnet/aspnetcore/tree/main/src/Identity)
- [ASP.NET Core Identity Default Flow Implementation with Page](https://github.com/dotnet/aspnetcore/blob/main/src/Identity/UI/src/Areas/Identity/Pages/V5/Account/ExternalLogin.cshtml.cs)

<sub> Work In Progress, Made With ❤️ By Ah...</sub>
