package server

import (
	"errors"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5/pgconn"
	database "gitlab.com/tednaaa/moner/internal/database/sqlc"
	"gitlab.com/tednaaa/moner/internal/tokens"
	"gitlab.com/tednaaa/moner/internal/utils"
)

const jwtDuration = time.Hour * 24 * 3

type UserResponse struct {
	Email    string `json:"email"`
	Username string `json:"username"`
}

type CreateUserDto struct {
	Email    string `json:"email" binding:"required,min=6,email"`
	Username string `json:"username" binding:"required,min=6,alphanum"`
	Password string `json:"password" binding:"required,min=6"`
}

type AuthUserDto struct {
	EmailOrUsername string `json:"emailOrUsername" binding:"required,min=6"`
	Password        string `json:"password" binding:"required,min=6"`
}

func (s *Server) setupUserRouter(router *gin.RouterGroup) {
	userGroup := router.Group("/user")

	userGroup.POST("", s.createUser)
	userGroup.POST("/authorize", s.authorizeUser)
	userGroup.GET("/logout", s.logoutUser)

	userGroup.Use(AuthMiddleware(s.jwtMaker))
	userGroup.GET("", s.getUser)
}

func (s *Server) createUser(ctx *gin.Context) {
	var createUserDto CreateUserDto
	if err := ctx.ShouldBindJSON(&createUserDto); err != nil {
		BadRequestError(ctx, err, "invalid request data")
		return
	}

	hashedPassword, err := utils.HashPassword(createUserDto.Password)
	if err != nil {
		InternalServerError(ctx, err, "internal error")
		return
	}

	arg := database.CreateUserParams{
		Email:    createUserDto.Email,
		Username: createUserDto.Username,
		Password: hashedPassword,
	}

	user, err := s.store.CreateUser(ctx, arg)
	if err != nil {
		if pgErr := (*pgconn.PgError)(nil); errors.As(err, &pgErr) {
			switch pgErr.ConstraintName {
			case "users_email_key":
				BadRequestError(ctx, err, "email already exists")
				return
			case "users_username_key":
				BadRequestError(ctx, err, "username already exists")
				return
			}
		}

		InternalServerError(ctx, err, "internal error")
		return
	}

	generateJwtToken(ctx, *s.jwtMaker, user.Email, user.Username)

	response := UserResponse{
		Email:    user.Email,
		Username: user.Username,
	}
	ctx.JSON(http.StatusCreated, response)
}

func (s *Server) authorizeUser(ctx *gin.Context) {
	var authUserDto AuthUserDto
	if err := ctx.ShouldBindJSON(&authUserDto); err != nil {
		BadRequestError(ctx, err, "invalid request data")
		return
	}

	arg := database.GetUserByEmailOrUsernameParams{
		Email:    authUserDto.EmailOrUsername,
		Username: authUserDto.EmailOrUsername,
	}
	user, err := s.store.GetUserByEmailOrUsername(ctx, arg)
	if err != nil {
		InternalServerError(ctx, err, "invalid email or username")
		return
	}

	err = utils.CheckPassword(authUserDto.Password, user.Password)
	if err != nil {
		BadRequestError(ctx, err, "invalid password")
		return
	}

	generateJwtToken(ctx, *s.jwtMaker, user.Email, user.Username)

	response := UserResponse{
		Email:    user.Email,
		Username: user.Username,
	}
	ctx.JSON(http.StatusOK, response)
}

func (s *Server) getUser(ctx *gin.Context) {
	userValue, exists := ctx.Get(authPayloadKey)
	if exists {
		user, ok := userValue.(*tokens.Payload)
		if !ok {
			UnauthorizedError(ctx, nil, "invalid token")
			return
		}

		response := UserResponse{
			Email:    user.Email,
			Username: user.Username,
		}
		ctx.JSON(http.StatusOK, response)
	} else {
		UnauthorizedError(ctx, nil, "invalid token")
	}
}

func (s *Server) logoutUser(ctx *gin.Context) {
	ctx.SetCookie("accessToken", "", -1, "/", "localhost", false, true)
	ctx.Status(http.StatusNoContent)
}

func generateJwtToken(ctx *gin.Context, jwtMaker tokens.JWTMaker, email, username string) {
	token, err := jwtMaker.CreateToken(email, username, jwtDuration)
	if err != nil {
		InternalServerError(ctx, err, "internal error")
		return
	}
	ctx.SetCookie("accessToken", token, int(jwtDuration.Milliseconds()), "/", "localhost", false, true)
}

// func (u *userHandler) RecoverPassword(ctx *gin.Context) {}
// func (u *userHandler) ChangePassword(ctx *gin.Context) {}
