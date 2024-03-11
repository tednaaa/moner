package server

import (
	"time"

	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog"
)

func RequestLogger(logger zerolog.Logger) gin.HandlerFunc {
	return func(ctx *gin.Context) {
		start := time.Now()
		path := ctx.Request.URL.Path
		method := ctx.Request.Method

		ctx.Next()

		latency := time.Since(start)
		statusCode := ctx.Writer.Status()

		logger.Info().
			Str("path", path).
			Str("method", method).
			Int("status_code", statusCode).
			Dur("latency", latency).
			Send()
	}
}
