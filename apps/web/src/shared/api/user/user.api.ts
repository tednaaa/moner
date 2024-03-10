import { apiInstance } from "../instance";
import type { ApiErrorResponse } from "../types";
import type { ApiAuthorizeUserDto, ApiCreateUserDto, ApiUserResponse } from "./user.types";

export function apiCreateUser({ email, username, password }: ApiCreateUserDto) {
  return apiInstance.post<ApiUserResponse | ApiErrorResponse>('/user', { email, username, password })
}

export function apiAuthorizeUser({ emailOrUsername, password }: ApiAuthorizeUserDto) {
  return apiInstance.post<ApiUserResponse>('/user/authorize', { emailOrUsername, password })
}

export function apiGetUser() {
  return apiInstance.get<ApiUserResponse>('/user')
}

export function apiLogoutUser() {
  return apiInstance.get('/user/logout')
}
