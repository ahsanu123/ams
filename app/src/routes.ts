import { index, layout, route, type RouteConfig } from "@react-router/dev/routes";

export enum AppRoutes {
  PagePrefix = "/",

  AccoutancyPage = "accoutancy-page",
  ReportPage = "report-page",

}

export default [
  layout("./layout/MainLayout.tsx", [
    index("./page/DashboardPage.tsx", {
      id: AppRoutes.PagePrefix
    }),
    route(`${AppRoutes.PagePrefix}${AppRoutes.AccoutancyPage}`, "./page/AccoutancyPage.tsx", {
      id: AppRoutes.AccoutancyPage
    }),
    route(`${AppRoutes.PagePrefix}${AppRoutes.ReportPage}`, "./page/ReportPage.tsx", {
      id: AppRoutes.ReportPage
    })
  ]),
  // layout("./layout/PageContainer.tsx", [
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.ClaimPage}`, "./page/ClaimPage.tsx"),
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.RolePage}`, "./page/RolePage.tsx"),
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.SignupPage}`, "./page/SignupPage.tsx"),
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.SigninPage}`, "./page/SigninPage.tsx"),
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.UserRolePage}/:userName`, "./page/UserRolePage.tsx"),
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.SuperAdminPage}`, "./page/SuperAdminPage.tsx"),
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.CampaignManagerPage}`, "./page/CampaignManagerPage.tsx"),
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.AMSAdminPage}`, "./page/AMSAdminPage.tsx"),
  //   route(`${AppRoutes.PagePrefix}${AppRoutes.NotAllowedOrNotFound}`, "./page/NotAllowedOrNotFoundPage.tsx"),
  // ])
] satisfies RouteConfig;
