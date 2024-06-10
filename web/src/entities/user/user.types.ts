export interface RegisterUserDto {
  email: string;
  username: string;
  password: string;
}

export interface LoginUserDto {
  login: string;
  password: string;
}

export interface UserResponse {
  id: number;
  email: string;
  username: string;
  createdAt: string;
}

export interface PublicUserResponse {
  id: number;
  email: string;
  username: string;
  name: string;
  avatar: string;
  occupation: string;
  isFollowing: boolean;
  followersCount: number;
  followingCount: number;
}
