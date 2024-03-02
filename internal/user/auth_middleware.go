package user

import (
	"github.com/gin-gonic/gin"
	"gitlab.com/tednaaa/moner/internal/api"
	"gitlab.com/tednaaa/moner/internal/tokens"
)

const authPayloadKey = "authorization_payload"

func AuthMiddleware() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		token, err := ctx.Cookie("accessToken")
		if err != nil {
			api.UnauthorizedError(ctx, err, "invalid token")
			ctx.Abort()
			return
		}

		user, err := tokens.VerifyToken(token)
		if err != nil {
			api.UnauthorizedError(ctx, err, "invalid token")
			ctx.Abort()
			return
		}

		ctx.Set(authPayloadKey, user)
		ctx.Next()
	}
}
