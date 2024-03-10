package user

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5/pgconn"
	"gitlab.com/tednaaa/moner/internal/api"
	database "gitlab.com/tednaaa/moner/internal/database/sqlc"
	"gitlab.com/tednaaa/moner/internal/tokens"
	"gitlab.com/tednaaa/moner/internal/utils"
)

type userHandler struct {
	queries database.Queries
}

func NewUserHandler(api *gin.RouterGroup, queries database.Queries) {
	userHandler := &userHandler{queries: queries}

	userGroup := api.Group("/user")
	userGroup.POST("", userHandler.create)
	userGroup.POST("/authorize", userHandler.authorize)
	userGroup.GET("/logout", userHandler.logout)

	authorizedGroup := userGroup.Use(AuthMiddleware())
	authorizedGroup.GET("", userHandler.get)
}

const jwtDuration = time.Hour * 24 * 3

func (u *userHandler) create(ctx *gin.Context) {
	var createUserDto createUserDto
	if err := ctx.ShouldBindJSON(&createUserDto); err != nil {
		api.BadRequestError(ctx, err, "invalid request data")
		return
	}

	hashedPassword, err := utils.HashPassword(createUserDto.Password)
	if err != nil {
		api.InternalServerError(ctx, err, "internal error")
		return
	}

	arg := database.CreateUserParams{
		Email:    createUserDto.Email,
		Username: createUserDto.Username,
		Password: hashedPassword,
	}

	user, err := u.queries.CreateUser(ctx, arg)
	if err != nil {
		if pgErr, ok := err.(*pgconn.PgError); ok {
			switch pgErr.ConstraintName {
			case "users_email_key":
				api.BadRequestError(ctx, err, "email already exists")
				return
			case "users_username_key":
				api.BadRequestError(ctx, err, "username already exists")
				return
			}
		}

		api.InternalServerError(ctx, err, "internal error")
		return
	}

	generateJwtToken(ctx, user.Email, user.Username)

	response := userResponse{
		Email:    user.Email,
		Username: user.Username,
	}
	ctx.JSON(http.StatusCreated, response)
}

func (u *userHandler) authorize(ctx *gin.Context) {
	var authUserDto authUserDto
	if err := ctx.ShouldBindJSON(&authUserDto); err != nil {
		api.BadRequestError(ctx, err, "invalid request data")
		return
	}

	arg := database.GetUserByEmailOrUsernameParams{
		Email:    authUserDto.EmailOrUsername,
		Username: authUserDto.EmailOrUsername,
	}
	user, err := u.queries.GetUserByEmailOrUsername(ctx, arg)
	if err != nil {
		api.InternalServerError(ctx, err, "invalid credentials")
		return
	}

	err = utils.CheckPassword(authUserDto.Password, user.Password)
	if err != nil {
		api.BadRequestError(ctx, err, "invalid password")
		return
	}

	generateJwtToken(ctx, user.Email, user.Username)

	response := userResponse{
		Email:    user.Email,
		Username: user.Username,
	}
	ctx.JSON(http.StatusOK, response)
}

func (u *userHandler) get(ctx *gin.Context) {
	userValue, exists := ctx.Get(authPayloadKey)
	if exists {
		user := userValue.(*tokens.Payload)
		response := userResponse{
			Email:    user.Email,
			Username: user.Username,
		}
		ctx.JSON(http.StatusOK, response)
	} else {
		api.UnauthorizedError(ctx, nil, "invalid token")
	}
}

func (u *userHandler) logout(ctx *gin.Context) {
	ctx.SetCookie("accessToken", "", -1, "/", "localhost", false, true)
	ctx.Status(http.StatusNoContent)
}

func generateJwtToken(ctx *gin.Context, email, username string) {
	token, err := tokens.CreateToken(email, username, jwtDuration)
	if err != nil {
		api.InternalServerError(ctx, err, "internal error")
		return
	}
	ctx.SetCookie("accessToken", token, int(jwtDuration.Milliseconds()), "/", "localhost", false, true)
}

// func (u *userHandler) RecoverPassword(ctx *gin.Context) {}
// func (u *userHandler) ChangePassword(ctx *gin.Context) {}
