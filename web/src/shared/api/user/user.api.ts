import { apiInstance } from "../instance";
import type { ApiLoginUserDto, ApiRegisterUserDto, ApiUserResponse } from "./user.types";

export function apiRegisterUser({ email, username, password }: ApiRegisterUserDto) {
  return apiInstance.post<ApiUserResponse>('/users/register', { email, username, password })
}

export function apiLoginUser({ login, password }: ApiLoginUserDto) {
  return apiInstance.post<ApiUserResponse>('/users/login', { login, password })
}

export function apiGetUser() {
  return apiInstance.get<ApiUserResponse>('/users/me')
}

export function apiLogoutUser() {
  return apiInstance.get('/users/logout')
}
