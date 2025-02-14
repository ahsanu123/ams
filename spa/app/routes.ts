import { index, layout, route, type RouteConfig } from "@react-router/dev/routes";

export enum AppRoutes {
  PagePrefix = "page/",

  AMSAdminPage = "ams-admin-page",
  ClaimPage = "claim-page",
  RolePage = "role-page",
  SigninPage = "signin-page",
  SignupPage = "signup-page",
  ListUserPage = "list-user-page",
  UserRolePage = "user-role-age",
  SuperAdminPage = "super-admin-page",
  CampaignManagerPage = "campaign-manager-page",
  NotAllowedOrNotFound = "not-allowed-or-not-found",
}

export default [
  layout("./layout/HomePage.tsx", [
    index("./page/DashboardPage.tsx")
  ]),
  layout("./layout/PageContainer.tsx", [
    route(`${AppRoutes.PagePrefix}${AppRoutes.ClaimPage}`, "./page/ClaimPage.tsx"),
    route(`${AppRoutes.PagePrefix}${AppRoutes.RolePage}`, "./page/RolePage.tsx"),
    route(`${AppRoutes.PagePrefix}${AppRoutes.SignupPage}`, "./page/SignupPage.tsx"),
    route(`${AppRoutes.PagePrefix}${AppRoutes.SigninPage}`, "./page/SigninPage.tsx"),
    route(`${AppRoutes.PagePrefix}${AppRoutes.UserRolePage}/:userName`, "./page/UserRolePage.tsx"),
    route(`${AppRoutes.PagePrefix}${AppRoutes.SuperAdminPage}`, "./page/SuperAdminPage.tsx"),
    route(`${AppRoutes.PagePrefix}${AppRoutes.CampaignManagerPage}`, "./page/CampaignManagerPage.tsx"),
    route(`${AppRoutes.PagePrefix}${AppRoutes.AMSAdminPage}`, "./page/AMSAdminPage.tsx"),
    route(`${AppRoutes.PagePrefix}${AppRoutes.NotAllowedOrNotFound}`, "./page/NotAllowedOrNotFoundPage.tsx"),
  ])
] satisfies RouteConfig;
