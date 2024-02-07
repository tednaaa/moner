package user

import "errors"

var (
	PasswordNotEmptyError  = errors.New("password should not be empty")
	UserAlreadyExistsError = errors.New("user already exists")
	UserNotFoundError      = errors.New("user not found")
)
