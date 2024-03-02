import { apiInstance } from "../instance";

interface UserResponse {
  email: string;
  username: string;
}

export function apiCreateUser(email: string, username: string, password: string) {
  return apiInstance.post<UserResponse>('/user', { email, username, password })
}

export function apiAuthorizeUser(emailOrUsername: string, password: string) {
  return apiInstance.post<UserResponse>('/user/authorize', { emailOrUsername, password })
}

export function apiGetUser() {
  return apiInstance.get<UserResponse>('/user')
}
