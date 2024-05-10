export interface ApiRegisterUserDto {
  email: string
  username: string
  password: string
}

export interface ApiLoginUserDto {
  login: string
  password: string
}

export interface ApiUserResponse {
  id: number;
  email: string;
  username: string;
}
