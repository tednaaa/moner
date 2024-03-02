package user

type userResponse struct {
	Email    string `json:"email"`
	Username string `json:"username"`
}

type createUserDto struct {
	Email    string `json:"email" binding:"required,min=6,email"`
	Username string `json:"username" binding:"required,min=6,alphanum"`
	Password string `json:"password" binding:"required,min=6"`
}

type authUserDto struct {
	EmailOrUsername string `json:"emailOrUsername" binding:"required,min=6"`
	Password        string `json:"password" binding:"required,min=6"`
}
