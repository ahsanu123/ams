import { index, layout, prefix, route, type RouteConfig } from "@react-router/dev/routes";

// Order in this enum is matter
export enum AppRoutes {
  Root = "/",

  ReportPage = "report-page",
  AccoutancyPage = "accoutancy-page",
  EditDataPage = "edit-data-page",
  AdminPage = "admin-page",
  EditPickingRecordPage = "edit-picking-record-page",
  MainAdminPage = "main-admin-page"
}

export enum SecretRoutes {
  AdminGuardPage = "admin-guard-page",
}

export enum AdminRoutes {
  AdminRoot = "/admin",

  ReportPage = "/report-page",
  MakePaymentPage = "/make-payment-page",
  ListPaymentPage = "/list-payment-page",
}

export default [
  // =========================================================
  // Customer Routes
  // =========================================================
  layout("./layout/MainLayout.tsx", [
    index("./page/customer-taking-page/CustomerTakingPage.tsx", {
      id: AppRoutes.Root
    }),

    route(`${SecretRoutes.AdminGuardPage}`, "./page/admin-guard-page/AdminGuardPage.tsx", {
      id: SecretRoutes.AdminGuardPage
    }),

  ]),

  // =========================================================
  // Admin Routes
  // =========================================================
  ...prefix(AdminRoutes.AdminRoot, [
    layout("./layout/AdminLayout.tsx", [

      index("./page/admin-page/AdminPage.tsx", {
        id: AdminRoutes.AdminRoot,
      }),

      route(AdminRoutes.MakePaymentPage, "./page/make-payment-pages/MakePaymentPages.tsx", {
        id: `${AdminRoutes.AdminRoot}${AdminRoutes.MakePaymentPage}`
      }),

      route(AdminRoutes.ListPaymentPage, "./page/list-payment-page/ListPaymentPage.tsx", {
        id: `${AdminRoutes.AdminRoot}${AdminRoutes.ListPaymentPage}`
      }),

    ]),

  ])

] satisfies RouteConfig;
