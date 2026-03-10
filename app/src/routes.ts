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

  ReportPage = "/report",
  PaymentPage = "/payment",
  ListPaymentPage = "/list-payment",
  UpdateTakingRecord = "/update-taking-record",
  CustomerManagementPage = "/customer-management",
  PricesPage = "/prices",
  GreetPage = "/greeting"
}

export default [
  layout("./layouts/MainLayout.tsx", [
    index("./pages/main-pages/MainPage.tsx", {
      id: AppRoutes.Root
    }),

    ...prefix(AdminRoutes.AdminRoot, [

      route(AdminRoutes.CustomerManagementPage, "./pages/customer-management-page/CustomerManagementPage.tsx", {
        id: `${AdminRoutes.AdminRoot}${AdminRoutes.CustomerManagementPage}`
      }),

      route(AdminRoutes.PaymentPage, "./pages/payment-page/PaymentPage.tsx", {
        id: `${AdminRoutes.AdminRoot}${AdminRoutes.PaymentPage}`
      }),

      route(AdminRoutes.PricesPage, "./pages/prices-page/PricesPage.tsx", {
        id: `${AdminRoutes.AdminRoot}${AdminRoutes.PricesPage}`
      }),

      route(AdminRoutes.GreetPage, "./pages/admin-greet-page/AdminGreetPage.tsx", {
        id: `${AdminRoutes.AdminRoot}${AdminRoutes.GreetPage}`
      }),

    ])

  ]),
  // =========================================================
  // Customer Routes
  // =========================================================
  // layout("./layout/MainLayout.tsx", [
  //   index("./page/customer-taking-page/CustomerTakingPage.tsx", {
  //     id: AppRoutes.Root
  //   }),
  //
  //   route(`${SecretRoutes.AdminGuardPage}`, "./page/admin-guard-page/AdminGuardPage.tsx", {
  //     id: SecretRoutes.AdminGuardPage
  //   }),
  //
  // ]),

  // =========================================================
  // Admin Routes
  // =========================================================
  // ...prefix(AdminRoutes.AdminRoot, [
  //   layout("./layout/AdminLayout.tsx", [
  //
  //     index("./page/admin-page/AdminPage.tsx", {
  //       id: AdminRoutes.AdminRoot,
  //     }),
  //
  //     route(AdminRoutes.MakePaymentPage, "./page/make-payment-pages/MakePaymentPages.tsx", {
  //       id: `${AdminRoutes.AdminRoot}${AdminRoutes.MakePaymentPage}`
  //     }),
  //
  //     route(AdminRoutes.ListPaymentPage, "./page/list-payment-page/ListPaymentPage.tsx", {
  //       id: `${AdminRoutes.AdminRoot}${AdminRoutes.ListPaymentPage}`
  //     }),
  //
  //     route(AdminRoutes.UpdateTakingRecord, "./page/update-taking-record-page/UpdateTakingRecordPage.tsx", {
  //       id: `${AdminRoutes.AdminRoot}${AdminRoutes.UpdateTakingRecord}`
  //     }),
  //
  //     route(AdminRoutes.UserManagementPage, "./page/user-management-pages/UserManagementPage.tsx", {
  //       id: `${AdminRoutes.AdminRoot}${AdminRoutes.UserManagementPage}`
  //     }),
  //
  //   ]),
  //
  // ])

] satisfies RouteConfig;
