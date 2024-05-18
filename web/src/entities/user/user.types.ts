export interface RegisterUserDto {
  email: string
  username: string
  password: string
}

export interface LoginUserDto {
  login: string
  password: string
}

export interface UserResponse {
  id: number
  email: string
  username: string
  created_at: string
  updated_at: string
}

export interface PublicUserResponse {
  id: number
  email: string
  username: string
}
