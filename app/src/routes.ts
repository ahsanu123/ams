import { index, layout, route, type RouteConfig } from "@react-router/dev/routes";

// Order in this enum is matter
export enum AppRoutes {
  PagePrefix = "/",

  ReportPage = "report-page",
  AccoutancyPage = "accoutancy-page",
  EditDataPage = "edit-data-page",
  AdminPage = "admin-page"
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
    }),
    route(`${AppRoutes.PagePrefix}${AppRoutes.EditDataPage}`, "./page/EditDataPage.tsx", {
      id: AppRoutes.EditDataPage
    }),
    route(`${AppRoutes.PagePrefix}${AppRoutes.AdminPage}`, "./page/AdminPage.tsx", {
      id: AppRoutes.AdminPage
    }),
  ]),
] satisfies RouteConfig;
