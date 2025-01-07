// This file is auto-generated by @hey-api/openapi-ts

import {
  createClient,
  createConfig,
  type OptionsLegacyParser,
} from "@hey-api/client-fetch";
import type {
  GetAccountExternalLoginData,
  GetAccountExternalLoginError,
  GetAccountExternalLoginResponse,
  GetAccountExternalLoginCallbackData,
  GetAccountExternalLoginCallbackError,
  GetAccountExternalLoginCallbackResponse,
  GetAccountExternalAuthenticationProviderInfoError,
  GetAccountExternalAuthenticationProviderInfoResponse,
  GetAmpasDailyAddAmpasData,
  GetAmpasDailyAddAmpasError,
  GetAmpasDailyAddAmpasResponse,
  PostAmpasDailySummaryInfoData,
  PostAmpasDailySummaryInfoError,
  PostAmpasDailySummaryInfoResponse,
  PostAmpasDailyUserSummaryInfoData,
  PostAmpasDailyUserSummaryInfoError,
  PostAmpasDailyUserSummaryInfoResponse,
  GetAmpasPricingCurrentPriceError,
  GetAmpasPricingCurrentPriceResponse,
  GetAmpasPricingChangePriceData,
  GetAmpasPricingChangePriceError,
  GetAmpasPricingChangePriceResponse,
  PostAmpasPricingMonthlyBillData,
  PostAmpasPricingMonthlyBillError,
  PostAmpasPricingMonthlyBillResponse,
  PostAmpasPricingMonthlyInformationData,
  PostAmpasPricingMonthlyInformationError,
  PostAmpasPricingMonthlyInformationResponse,
  PostAmpasPricingSetPaidStatusData,
  PostAmpasPricingSetPaidStatusError,
  PostAmpasPricingSetPaidStatusResponse,
  GetAmpasReporterAmpasReportData,
  GetAmpasReporterAmpasReportError,
  GetAmpasReporterAmpasReportResponse,
  PostCampaignCreateCampaignData,
  PostCampaignCreateCampaignError,
  PostCampaignCreateCampaignResponse,
  PostCampaignUpdateCampaignData,
  PostCampaignUpdateCampaignError,
  PostCampaignUpdateCampaignResponse,
  GetCampaignCampaignsError,
  GetCampaignCampaignsResponse,
  PostLocalAccountRegisterData,
  PostLocalAccountRegisterError,
  PostLocalAccountRegisterResponse,
  PostLocalAccountLoginWithoutPasswordData,
  PostLocalAccountLoginWithoutPasswordError,
  PostLocalAccountLoginWithoutPasswordResponse,
  PostLocalAccountLoginData,
  PostLocalAccountLoginError,
  PostLocalAccountLoginResponse,
  GetLocalAccountLogoutError,
  GetLocalAccountLogoutResponse,
  GetLocalAccountListUserError,
  GetLocalAccountListUserResponse,
  PostRoleManagerAddRoleForEmailByEmailData,
  PostRoleManagerAddRoleForEmailByEmailError,
  PostRoleManagerAddRoleForEmailByEmailResponse,
  PostRoleManagerRemoveRoleForUserByUserNameData,
  PostRoleManagerRemoveRoleForUserByUserNameError,
  PostRoleManagerRemoveRoleForUserByUserNameResponse,
  PostRoleManagerAddRoleForUserByUserNameData,
  PostRoleManagerAddRoleForUserByUserNameError,
  PostRoleManagerAddRoleForUserByUserNameResponse,
  PostRoleManagerCreateRoleData,
  PostRoleManagerCreateRoleError,
  PostRoleManagerCreateRoleResponse,
  GetRoleManagerGetRolesError,
  GetRoleManagerGetRolesResponse,
  GetRoleManagerGetRoleForEmailByEmailData,
  GetRoleManagerGetRoleForEmailByEmailError,
  GetRoleManagerGetRoleForEmailByEmailResponse,
  GetRoleManagerGetRoleForUserByUserNameData,
  GetRoleManagerGetRoleForUserByUserNameError,
  GetRoleManagerGetRoleForUserByUserNameResponse,
  GetRoleManagerIsUserInRoleData,
  GetRoleManagerIsUserInRoleError,
  GetRoleManagerIsUserInRoleResponse,
  GetRoleManagerGetUserForRoleData,
  GetRoleManagerGetUserForRoleError,
  GetRoleManagerGetUserForRoleResponse,
  GetUserInfoWhoAmIError,
  GetUserInfoWhoAmIResponse,
  PostUserManagerUserDetailData,
  PostUserManagerUserDetailError,
  PostUserManagerUserDetailResponse,
  PostUserManagerCreateUserData,
  PostUserManagerCreateUserError,
  PostUserManagerCreateUserResponse,
  GetUserManagerListUsersError,
  GetUserManagerListUsersResponse,
  PostUserManagerUpdateUserData,
  PostUserManagerUpdateUserError,
  PostUserManagerUpdateUserResponse,
  DeleteUserManagerDeleteUserData,
  DeleteUserManagerDeleteUserError,
  DeleteUserManagerDeleteUserResponse,
  GetUserManagerSignOutError,
  GetUserManagerSignOutResponse,
} from "./types.gen";

export const client = createClient(createConfig());

export class AccountService {
  public static getAccountExternalLogin<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<GetAccountExternalLoginData, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetAccountExternalLoginResponse,
      GetAccountExternalLoginError,
      ThrowOnError
    >({
      ...options,
      url: "/Account/ExternalLogin",
    });
  }

  public static getAccountExternalLoginCallback<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<
      GetAccountExternalLoginCallbackData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).get<
      GetAccountExternalLoginCallbackResponse,
      GetAccountExternalLoginCallbackError,
      ThrowOnError
    >({
      ...options,
      url: "/Account/ExternalLoginCallback",
    });
  }

  public static getAccountExternalAuthenticationProviderInfo<
    ThrowOnError extends boolean = false
  >(options?: OptionsLegacyParser<unknown, ThrowOnError>) {
    return (options?.client ?? client).get<
      GetAccountExternalAuthenticationProviderInfoResponse,
      GetAccountExternalAuthenticationProviderInfoError,
      ThrowOnError
    >({
      ...options,
      url: "/Account/external-authentication-provider-info",
    });
  }
}

export class AmpasDailyService {
  public static getAmpasDailyAddAmpas<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<GetAmpasDailyAddAmpasData, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetAmpasDailyAddAmpasResponse,
      GetAmpasDailyAddAmpasError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasDaily/add-ampas",
    });
  }

  public static postAmpasDailySummaryInfo<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<PostAmpasDailySummaryInfoData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostAmpasDailySummaryInfoResponse,
      PostAmpasDailySummaryInfoError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasDaily/summary-info",
    });
  }

  public static postAmpasDailyUserSummaryInfo<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<
      PostAmpasDailyUserSummaryInfoData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).post<
      PostAmpasDailyUserSummaryInfoResponse,
      PostAmpasDailyUserSummaryInfoError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasDaily/user-summary-info",
    });
  }
}

export class AmpasPricingService {
  public static getAmpasPricingCurrentPrice<
    ThrowOnError extends boolean = false
  >(options?: OptionsLegacyParser<unknown, ThrowOnError>) {
    return (options?.client ?? client).get<
      GetAmpasPricingCurrentPriceResponse,
      GetAmpasPricingCurrentPriceError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasPricing/current-price",
    });
  }

  public static getAmpasPricingChangePrice<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<GetAmpasPricingChangePriceData, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetAmpasPricingChangePriceResponse,
      GetAmpasPricingChangePriceError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasPricing/change-price",
    });
  }

  public static postAmpasPricingMonthlyBill<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<PostAmpasPricingMonthlyBillData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostAmpasPricingMonthlyBillResponse,
      PostAmpasPricingMonthlyBillError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasPricing/monthly-bill",
    });
  }

  public static postAmpasPricingMonthlyInformation<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<
      PostAmpasPricingMonthlyInformationData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).post<
      PostAmpasPricingMonthlyInformationResponse,
      PostAmpasPricingMonthlyInformationError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasPricing/monthly-information",
    });
  }

  public static postAmpasPricingSetPaidStatus<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<
      PostAmpasPricingSetPaidStatusData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).post<
      PostAmpasPricingSetPaidStatusResponse,
      PostAmpasPricingSetPaidStatusError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasPricing/set-paid-status",
    });
  }
}

export class AmpasReporterService {
  public static getAmpasReporterAmpasReport<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<GetAmpasReporterAmpasReportData, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetAmpasReporterAmpasReportResponse,
      GetAmpasReporterAmpasReportError,
      ThrowOnError
    >({
      ...options,
      url: "/AmpasReporter/ampas-report",
    });
  }
}

export class CampaignService {
  public static postCampaignCreateCampaign<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<PostCampaignCreateCampaignData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostCampaignCreateCampaignResponse,
      PostCampaignCreateCampaignError,
      ThrowOnError
    >({
      ...options,
      url: "/Campaign/create-campaign",
    });
  }

  public static postCampaignUpdateCampaign<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<PostCampaignUpdateCampaignData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostCampaignUpdateCampaignResponse,
      PostCampaignUpdateCampaignError,
      ThrowOnError
    >({
      ...options,
      url: "/Campaign/update-campaign",
    });
  }

  public static getCampaignCampaigns<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<unknown, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetCampaignCampaignsResponse,
      GetCampaignCampaignsError,
      ThrowOnError
    >({
      ...options,
      url: "/Campaign/campaigns",
    });
  }
}

export class LocalAccountService {
  public static postLocalAccountRegister<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<PostLocalAccountRegisterData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostLocalAccountRegisterResponse,
      PostLocalAccountRegisterError,
      ThrowOnError
    >({
      ...options,
      url: "/LocalAccount/register",
    });
  }

  public static postLocalAccountLoginWithoutPassword<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<
      PostLocalAccountLoginWithoutPasswordData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).post<
      PostLocalAccountLoginWithoutPasswordResponse,
      PostLocalAccountLoginWithoutPasswordError,
      ThrowOnError
    >({
      ...options,
      url: "/LocalAccount/login-without-password",
    });
  }

  public static postLocalAccountLogin<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<PostLocalAccountLoginData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostLocalAccountLoginResponse,
      PostLocalAccountLoginError,
      ThrowOnError
    >({
      ...options,
      url: "/LocalAccount/login",
    });
  }

  public static getLocalAccountLogout<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<unknown, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetLocalAccountLogoutResponse,
      GetLocalAccountLogoutError,
      ThrowOnError
    >({
      ...options,
      url: "/LocalAccount/logout",
    });
  }

  public static getLocalAccountListUser<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<unknown, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetLocalAccountListUserResponse,
      GetLocalAccountListUserError,
      ThrowOnError
    >({
      ...options,
      url: "/LocalAccount/list-user",
    });
  }
}

export class RoleManagerService {
  public static postRoleManagerAddRoleForEmailByEmail<
    ThrowOnError extends boolean = false
  >(
    options: OptionsLegacyParser<
      PostRoleManagerAddRoleForEmailByEmailData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).post<
      PostRoleManagerAddRoleForEmailByEmailResponse,
      PostRoleManagerAddRoleForEmailByEmailError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/add-role-for-email/{email}",
    });
  }

  public static postRoleManagerRemoveRoleForUserByUserName<
    ThrowOnError extends boolean = false
  >(
    options: OptionsLegacyParser<
      PostRoleManagerRemoveRoleForUserByUserNameData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).post<
      PostRoleManagerRemoveRoleForUserByUserNameResponse,
      PostRoleManagerRemoveRoleForUserByUserNameError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/remove-role-for-user/{userName}",
    });
  }

  public static postRoleManagerAddRoleForUserByUserName<
    ThrowOnError extends boolean = false
  >(
    options: OptionsLegacyParser<
      PostRoleManagerAddRoleForUserByUserNameData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).post<
      PostRoleManagerAddRoleForUserByUserNameResponse,
      PostRoleManagerAddRoleForUserByUserNameError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/add-role-for-user/{userName}",
    });
  }

  public static postRoleManagerCreateRole<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<PostRoleManagerCreateRoleData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostRoleManagerCreateRoleResponse,
      PostRoleManagerCreateRoleError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/create-role",
    });
  }

  public static getRoleManagerGetRoles<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<unknown, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetRoleManagerGetRolesResponse,
      GetRoleManagerGetRolesError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/get-roles",
    });
  }

  public static getRoleManagerGetRoleForEmailByEmail<
    ThrowOnError extends boolean = false
  >(
    options: OptionsLegacyParser<
      GetRoleManagerGetRoleForEmailByEmailData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).get<
      GetRoleManagerGetRoleForEmailByEmailResponse,
      GetRoleManagerGetRoleForEmailByEmailError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/get-role-for-email/{email}",
    });
  }

  public static getRoleManagerGetRoleForUserByUserName<
    ThrowOnError extends boolean = false
  >(
    options: OptionsLegacyParser<
      GetRoleManagerGetRoleForUserByUserNameData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).get<
      GetRoleManagerGetRoleForUserByUserNameResponse,
      GetRoleManagerGetRoleForUserByUserNameError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/get-role-for-user/{userName}",
    });
  }

  public static getRoleManagerIsUserInRole<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<GetRoleManagerIsUserInRoleData, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetRoleManagerIsUserInRoleResponse,
      GetRoleManagerIsUserInRoleError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/is-user-in-role",
    });
  }

  public static getRoleManagerGetUserForRole<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<
      GetRoleManagerGetUserForRoleData,
      ThrowOnError
    >
  ) {
    return (options?.client ?? client).get<
      GetRoleManagerGetUserForRoleResponse,
      GetRoleManagerGetUserForRoleError,
      ThrowOnError
    >({
      ...options,
      url: "/RoleManager/get-user-for-role",
    });
  }
}

export class UserInfoService {
  public static getUserInfoWhoAmI<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<unknown, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetUserInfoWhoAmIResponse,
      GetUserInfoWhoAmIError,
      ThrowOnError
    >({
      ...options,
      url: "/UserInfo/who-am-i",
    });
  }
}

export class UserManagerService {
  public static postUserManagerUserDetail<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<PostUserManagerUserDetailData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostUserManagerUserDetailResponse,
      PostUserManagerUserDetailError,
      ThrowOnError
    >({
      ...options,
      url: "/UserManager/user-detail",
    });
  }

  public static postUserManagerCreateUser<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<PostUserManagerCreateUserData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostUserManagerCreateUserResponse,
      PostUserManagerCreateUserError,
      ThrowOnError
    >({
      ...options,
      url: "/UserManager/create-user",
    });
  }

  public static getUserManagerListUsers<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<unknown, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetUserManagerListUsersResponse,
      GetUserManagerListUsersError,
      ThrowOnError
    >({
      ...options,
      url: "/UserManager/list-users",
    });
  }

  public static postUserManagerUpdateUser<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<PostUserManagerUpdateUserData, ThrowOnError>
  ) {
    return (options?.client ?? client).post<
      PostUserManagerUpdateUserResponse,
      PostUserManagerUpdateUserError,
      ThrowOnError
    >({
      ...options,
      url: "/UserManager/update-user",
    });
  }

  public static deleteUserManagerDeleteUser<
    ThrowOnError extends boolean = false
  >(
    options?: OptionsLegacyParser<DeleteUserManagerDeleteUserData, ThrowOnError>
  ) {
    return (options?.client ?? client).delete<
      DeleteUserManagerDeleteUserResponse,
      DeleteUserManagerDeleteUserError,
      ThrowOnError
    >({
      ...options,
      url: "/UserManager/delete-user",
    });
  }

  public static getUserManagerSignOut<ThrowOnError extends boolean = false>(
    options?: OptionsLegacyParser<unknown, ThrowOnError>
  ) {
    return (options?.client ?? client).get<
      GetUserManagerSignOutResponse,
      GetUserManagerSignOutError,
      ThrowOnError
    >({
      ...options,
      url: "/UserManager/sign-out",
    });
  }
}
