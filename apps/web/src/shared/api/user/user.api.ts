import { apiInstance } from "../instance";
import { apiRoutes } from "../api.routes";

export function apiCreateUser(email: string, username: string, password: string) {
  return apiInstance.post<{}>(apiRoutes.user, { email, username, password })
}
