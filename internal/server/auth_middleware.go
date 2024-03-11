package server

import (
	"github.com/gin-gonic/gin"
	"gitlab.com/tednaaa/moner/internal/tokens"
)

const authPayloadKey = "authorization_payload"

func AuthMiddleware(jwtMaker *tokens.JWTMaker) gin.HandlerFunc {
	return func(ctx *gin.Context) {
		token, err := ctx.Cookie("accessToken")
		if err != nil {
			UnauthorizedError(ctx, err, "invalid token")
			ctx.Abort()
			return
		}

		user, err := jwtMaker.VerifyToken(token)
		if err != nil {
			UnauthorizedError(ctx, err, "invalid token")
			ctx.Abort()
			return
		}

		ctx.Set(authPayloadKey, user)
		ctx.Next()
	}
}
