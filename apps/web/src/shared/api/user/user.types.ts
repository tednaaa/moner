export interface ApiCreateUserDto {
  email: string
  username: string
  password: string
}

export interface ApiAuthorizeUserDto {
  emailOrUsername: string
  password: string
}

export interface ApiUserResponse {
  email: string;
  username: string;
}
