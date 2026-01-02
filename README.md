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

 <img alt="logo" src="./docs/ams-logo.svg">


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

### 🎻 Current UI  

<img width="1322" height="610" alt="image" src="https://github.com/user-attachments/assets/ed91b689-87b3-451a-9770-bea744d44272" />
<img alt="gif" src="./docs/ams.gif">
  
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
- [read more on auto backup database](./scripts/Production-Auto-Backup.md)
- ams-prod.sqlite in root folder is empty database for production

 
## ⚓ Reference  

- [tauri](https://v2.tauri.app/)
- [Chakra UI](https://chakra-ui.com/)
- [Sea-ORM](https://www.sea-ql.org/SeaORM/)
- [CrossCompile](./docs/cross_compile.md)

<sub> Work In Progress, Made With ❤️ By Ah...</sub>
